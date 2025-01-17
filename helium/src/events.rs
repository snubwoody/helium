use std::{collections::HashMap, fmt::Debug};
use crystal::{Layout, Position};
use helium_core::position::Bounds;
use winit::event::WindowEvent;

/// Stores callback functions for [`Widget`]'s
pub struct EventContext{
	/// A map of callbacks with their widget `id`s
	callbacks:Vec<EventFn>
}

impl EventContext {
	pub fn new() -> Self{
		Self{callbacks:vec![]}
	}

	pub fn add(&mut self,callback:EventFn){
		self.callbacks.push(callback);
	}
}


pub enum EventFn {
    OnHover(String,Box<dyn FnMut()>),
    OnClick(String,Box<dyn FnMut()>),
}

impl EventFn {
	pub fn hover(id:&str,f:impl FnMut() + 'static) -> Self{
		Self::OnHover(id.to_string(), Box::new(f))
	}

	pub fn click(id:&str,f:impl FnMut() + 'static) -> Self{
		Self::OnClick(id.to_string(), Box::new(f))
	}

    fn run_hover(&mut self,widget_id:&str) {
        match self {
            Self::OnHover(id,func) => {
				if id == widget_id {
					(func)()
				}
			},
            _ => {},
        }
    }
 
    fn run_click(&mut self,widget_id:&str) {
        match self {
            Self::OnClick(id,func) => {
				if id == widget_id{
					(func)()
				}
			},
            _ => {},
        }
    }
}

impl Debug for EventFn {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::OnClick(id,_) => f.debug_tuple(format!("OnClick({id},_)").as_str()).finish(),
			Self::OnHover(id,_) => f.debug_tuple(format!("OnHover({id},_)").as_str()).finish(),
		}
	}
}

#[derive(Debug,Default,PartialEq,Eq, PartialOrd,Ord,Clone, Copy,Hash)]
enum ElementState{
	#[default]
	Default,
	Hovered,
	Clicked
}


/// Describes the state of a [`Widget`]
#[derive(Debug,Clone, PartialEq, Eq,PartialOrd,Ord)]
struct Element {
	id:String,
	previous_state:ElementState,
	state:ElementState,
}

impl Element {
	fn new(id:&str) -> Self{
		Self{
			id:String::from(id),
			previous_state:ElementState::Default,
			state:ElementState::Default,
		}
	}

	/// Set the element state to whatever it was previously
	fn roll_back(&mut self){
		self.state = self.previous_state;
	}
	
	/// Set the element state to `ElementState::Default`
	fn default(&mut self){
		self.previous_state = self.state;
		self.state = ElementState::Default;
	}

	/// Set the element state to `ElementState::Clicked`
	fn click(&mut self){
		self.previous_state = self.state;
		self.state = ElementState::Clicked;
	}

	/// Set the element state to `ElementState::Hovered`
	fn hover(&mut self){
		// FIXME
		self.previous_state = ElementState::Default;
		self.state = ElementState::Hovered;
	}
}

#[derive(Debug)]
pub struct EventManager {
    mouse_pos: Position,
	elements: Vec<Element>,
	callbacks:Vec<EventFn>
}

impl EventManager {
    pub fn new(cx:EventContext,layout: &dyn Layout) -> Self {
		let elements:Vec<Element> = layout.iter().map(|l|Element::new(l.id())).collect();

		Self{
			elements,
			mouse_pos:Position::default(),
			callbacks:cx.callbacks
		}
    }

	fn process_hover(&mut self,layout: &dyn Layout){
		let bounds = Bounds::new(layout.position(), layout.size());
		let mouse_pos = self.mouse_pos;
		let element = self.elements.iter_mut().find(|e|e.id == layout.id()).unwrap();

		if bounds.within(&mouse_pos){
			match element.state {
				ElementState::Default => {
					element.hover();
					for callback in &mut self.callbacks{
						callback.run_hover(layout.id());
					}
				},
				_ => {}
			}
		}else {
			element.default();
			return;
		}
	}

	fn process_mouse(
		&mut self,
		layout: &dyn Layout,
		state:&winit::event::ElementState,
		button:&winit::event::MouseButton
	){
		let element = self.elements.iter_mut().find(|e|e.id == layout.id()).unwrap();
		// TODO use right click only
		match state {
			&winit::event::ElementState::Pressed => {
				match element.state {
					ElementState::Default => {},
					ElementState::Hovered => {
						element.click();
						for callback in &mut self.callbacks{
							callback.run_click(layout.id());
						}
					},
					ElementState::Clicked => {}
				}
			}
			&winit::event::ElementState::Released => {
				// Not sure about this
				element.roll_back();
			}
		}
		
	}

	/// Process the incoming `WindowEvent` and dispatch events to [`Widget`]'s
    pub fn process(
        &mut self,
        event: &winit::event::WindowEvent,
        layout: &dyn Layout,
    ){
		// FIXME please handle the panics
        match event {
			WindowEvent::CursorMoved {position,..} => {
				self.mouse_pos = (*position).into();
                for layout in layout.iter() {
					self.process_hover(layout);
                }
            },
            WindowEvent::MouseInput {state,button,..} => {
				for layout in layout.iter() {
					self.process_mouse(layout,state, button);
                }
			},
            _ => {}
        }
    }
}


#[cfg(test)]
mod test{
	use super::*;
	use crystal::{EmptyLayout, Size};
	use winit::{
		dpi::PhysicalPosition, 
		event::{DeviceId, ElementState as WinitElementState, MouseButton}
	};

	#[test]
	fn event_and_widget_ids_match(){

	}

	#[test]
	fn mouse_position_updates(){
		let mut events = EventManager::new(EventContext::new(),&EmptyLayout::default());
		
		let device_id = unsafe {DeviceId::dummy()};
		let position = PhysicalPosition::new(50.0, 60.0);
		let cursor_event = WindowEvent::CursorMoved {device_id,position};

		events.process(&cursor_event, &EmptyLayout::default());
		assert_eq!(events.mouse_pos,position.into())
	}

	#[test]
	fn hover_state(){
		let mut layout = EmptyLayout::default();
		layout.size = Size::new(500.0, 500.0);
		let mut events = EventManager::new(EventContext::new(),&layout);

		let device_id = unsafe {DeviceId::dummy()};
		let position = PhysicalPosition::new(92.23, 63.2);

		let cursor_event = WindowEvent::CursorMoved {device_id,position};
		events.process(&cursor_event, &layout);

		assert_eq!(events.elements[0].state,ElementState::Hovered);
	}

	#[test]
	fn click_element_state(){
		let layout = EmptyLayout::default();
		let mut events = EventManager::new(EventContext::new(),&layout);

		let device_id = unsafe {DeviceId::dummy()};
		let click_event = WindowEvent::MouseInput { 
			device_id, 
			state: WinitElementState::Pressed, 
			button: MouseButton::Left
		};
		events.elements[0].state = ElementState::Hovered;
		events.process(&click_event, &layout);
		assert_eq!(events.elements[0].state,ElementState::Clicked);
		
		let click_event = WindowEvent::MouseInput { 
			device_id, 
			state: WinitElementState::Released, 
			button: MouseButton::Left
		};
		events.process(&click_event, &layout);
		assert_eq!(events.elements[0].state,ElementState::Hovered);
	}
}