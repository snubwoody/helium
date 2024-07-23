use std::marker;

use glium::{
	glutin::surface::WindowSurface, Display, Frame, Program, 
};
use winit::window::Window;
use crate::widgets::Widget;


pub enum StackDirection {
	Horizontal,
	Vertical
}

/// A [`Widget`] that arranges it's children either
/// horizontally or vertically.
pub struct Stack{
	pub y:i32,
	pub x:i32,
	pub width:i32,
	pub height:i32,
	pub spacing:i32,
	pub direction:StackDirection,
	pub children:Vec<Box<dyn Widget>>
}

//TODO there might be unnecessary mutability here
impl Widget for Stack {
	fn render(
		&mut self,
		display:&Display<WindowSurface>,
		frame:&mut Frame,
		window:&Window,
		program:&Program,
	){
		let mut offset = 0;
		self.children.iter_mut().for_each(|child|{
			let position = offset;
			child.render(display, frame, window, program);

			//TODO might cause issues due to setting the other position to 0
			match self.direction {
				StackDirection::Horizontal => {
					let size = child.size();
					offset += self.spacing + size[0];
					child.set_position(position, 0);
				},
				StackDirection::Vertical => {
					let size = child.size();
					offset += self.spacing + size[1];
					child.set_position(0, position);
				}
			}
		});
	}

	fn set_position(&mut self,x:i32,y:i32) {
		self.x = x;
		self.y = y;
	}

	fn size(&mut self) -> [i32;2] {
		return [self.width,self.height];
	}
}



#[macro_export]
/// A Phantom [`Widget`] that returns a stack with a horizontal direction
macro_rules! hstack {
	(
		spacing:$spacing:expr, 
		width:$width:expr,
		height:$height:expr,
		$($x:expr),
		*
	) => {
		{
			
            let mut children = Vec::new();

            $(
                children.push(Box::new($x) as Box<dyn Widget>);
            )*

            Stack{
				x:0,
				y:0,
				direction:StackDirection::Horizontal,
				width:$width,
				height:$height,
				spacing:$spacing,
				children:children
			}
        }
	};
}

#[macro_export]
/// A Phantom [`Widget`] that returns a stack with a vertical direction
macro_rules! vstack {
	(
		spacing:$spacing:expr, 
		width:$width:expr,
		height:$height:expr,
		$($x:expr),
		*
	) => {
		{
			
            let mut children = Vec::new();

            $(
                children.push(Box::new($x) as Box<dyn Widget>);
            )*

            Stack{
				x:0,
				y:0,
				direction:StackDirection::Vertical,
				width:$width,
				height:$height,
				spacing:$spacing,
				children:children
			}
        }
	};
}

