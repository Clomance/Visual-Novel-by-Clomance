use crate::*;

pub struct Button<'a>{
    button_base:ButtonDependent,
    glyphs:GlyphCache<'a>
}

impl<'a> Button<'a>{
    pub fn new(settings:ButtonSettings,mut glyphs:GlyphCache<'a>)->Button<'a>{
        Self{
            button_base:ButtonDependent::new(settings,&mut glyphs),
            glyphs:glyphs,
        }
    }

    pub fn clicked(&mut self)->bool{
        self.button_base.clicked()
    }
}

impl<'a> Drawable for Button<'a>{
    fn set_alpha_channel(&mut self,alpha:f32){
        self.button_base.set_alpha_channel(alpha);
    }

    fn draw(&mut self,context:&Context,g:&mut GlGraphics){
        self.button_base.draw(context,g,&mut self.glyphs)
    }
}

// Зависимая от шрифта кнопка (должно быть больше зависимостей)
pub struct ButtonDependent{
    base:ButtonBase,
    text:TextViewDependent, // Зависимый от шрифта текстовый блок
}

impl ButtonDependent{
    pub fn new(settings:ButtonSettings,glyphs:&mut GlyphCache)->ButtonDependent{
        let text_view_settings=TextViewSettings::new()
                .rect(settings.rect)
                .text_color(settings.text_color)
                .text(settings.text)
                .font_size(settings.font_size);
        Self{
            base:ButtonBase::new(settings.rect,settings.background_color),
            text:TextViewDependent::new(text_view_settings,glyphs),
        }
    }

    pub fn set_alpha_channel(&mut self,alpha:f32){
        self.base.set_alpha_channel(alpha);
        self.text.set_alpha_channel(alpha);
    }

    pub fn clicked(&mut self)->bool{
        self.base.released()
    }
    
    pub fn draw(&mut self,context:&Context,graphics:&mut GlGraphics,glyphs:&mut GlyphCache){
        self.base.draw(context,graphics);
        self.text.draw(context,graphics,glyphs);
    }
}

#[derive(Clone)]
pub struct ButtonSettings{
    pub rect:[f64;4],
    pub background_color:Color,
    pub text:String,
    pub font_size:u32,
    pub text_color:Color
}

impl ButtonSettings{
    pub fn new()->ButtonSettings{
        Self{
            rect:[0f64;4],
            background_color:Light_blue,
            text:String::new(),
            font_size:20,
            text_color:Black,
        }
    }

    pub fn rect(mut self,rect:[f64;4])->ButtonSettings{
        self.rect=rect;
        self
    }

    pub fn background_color(mut self,color:Color)->ButtonSettings{
        self.background_color=color;
        self
    }

    pub fn text(mut self,text:String)->ButtonSettings{
        self.text=text;
        self
    }

    pub fn font_size(mut self,size:u32)->ButtonSettings{
        self.font_size=size;
        self
    }
    
    pub fn text_color(mut self,color:Color)->ButtonSettings{
        self.text_color=color;
        self
    }
}

// Второе название JmyakButton - предложил Тимур Шайхинуров
// Кнопка, в которую вписывается крестик при нажатии
pub struct CheckButton{
    button_base:ButtonBase,
    tick_color:Color,
    ticked:bool
}

impl CheckButton{
    pub fn new(rect:[f64;4],background_color:Color,ticked:bool)->CheckButton{
        Self{
            button_base:ButtonBase::new(rect,background_color),
            tick_color:Red,
            ticked:ticked
        }
    }

    pub fn set_alpha_channel(&mut self,alpha:f32){
        self.button_base.set_alpha_channel(alpha)
    }

    pub fn clicked(&mut self)->bool{
        if self.button_base.released(){
            self.ticked=!self.ticked;
            true
        }
        else{
            false
        }
    }

    pub fn draw(&self,context:&Context,g:&mut GlGraphics){
        self.button_base.draw(context,g);
        if self.ticked{
            let line=Line::new(self.tick_color,1f64);
            
            line.draw(
                [
                    self.button_base.x1,
                    self.button_base.y1,
                    self.button_base.x2,
                    self.button_base.y2
                ],
                &context.draw_state,
                context.transform,
                g
            );

            line.draw(
                [
                    self.button_base.x1,
                    self.button_base.y2,
                    self.button_base.x2,
                    self.button_base.y1
                ],
                &context.draw_state,
                context.transform,
                g
            )
        }
    }
}

struct ButtonBase{
    x1:f64,
    y1:f64,
    x2:f64,
    y2:f64,
    width:f64,
    height:f64,
    rectangle:Rectangle,
}

impl ButtonBase{
    #[inline]
    pub fn new(rect:[f64;4],color:Color)->ButtonBase{
        Self{
            x1:rect[0],
            y1:rect[1],
            x2:rect[0]+rect[2],
            y2:rect[1]+rect[3],
            width:rect[2],
            height:rect[3],
            rectangle:Rectangle::new(color),
        }
    }

    #[inline] // Установка альфа-канала
    pub fn set_alpha_channel(&mut self,alpha:f32){
        self.rectangle.color[3]=alpha;
    }

    #[inline] // Проверка нажатия на кнопку и локальные действия
    pub fn pressed(&self)->bool{
        let position=unsafe{mouse_cursor.position()};
        let x=position[0];
        let y=position[1];

        self.x1<x && self.x2>x && self.y1<y && self.y2>y
    }

    #[inline] // Проверка отпущеная ли кнопка
    pub fn released(&self)->bool{
        let position=unsafe{mouse_cursor.position()};
        let x=position[0];
        let y=position[1];

        self.x1<x && self.x2>x && self.y1<y && self.y2>y
    }

    #[inline]
    pub fn draw(&self,context:&Context,g:&mut GlGraphics){
        let rect_pos=[self.x1,self.y1,self.width,self.height];
        self.rectangle.draw(rect_pos,&context.draw_state,context.transform,g);
    }
}