use cat_engine::glium::{
    DrawError,
    DrawParameters,
};

use cat_engine::graphics::{
    Graphics,
    DrawType,
    ObjectType
};

#[derive(Clone)]
pub struct DrawableObject{
    pub index:usize,
    pub object_type:ObjectType,
    pub draw_type:DrawType
}

impl DrawableObject{
    pub fn new(index:usize,object_type:ObjectType,draw_type:DrawType)->DrawableObject{
        Self{
            index,
            object_type,
            draw_type
        }
    }

    pub fn set_draw_type(&mut self,draw_type:DrawType){
        self.draw_type=draw_type
    }

    pub fn update_rotating_angle(&mut self,new_angle:f32){
        if let DrawType::Rotating((angle,_))=&mut self.draw_type{
            *angle=new_angle;
        }
    }

    pub fn draw(&self,draw_parameters:&DrawParameters,graphics:&mut Graphics)->Result<(),DrawError>{
        graphics.draw_object(self.index,self.object_type.clone(),self.draw_type.clone(),draw_parameters)
    }
}

pub trait Drawable{
    fn index(&self)->usize;
    fn object_type(&self)->ObjectType;
    fn draw_type(&self)->DrawType;
}