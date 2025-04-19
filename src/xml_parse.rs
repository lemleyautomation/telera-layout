use std::{collections::HashMap, fmt::Debug, fs, path::Path, str::FromStr};

use quick_xml::events::Event;
use quick_xml::reader::Reader;
use quick_xml::events::BytesStart;
use quick_xml::Decoder;
use strum_macros::{Display, EnumString};

use crate::{Color, ElementConfiguration, LayoutEngine, TextConfig};

#[derive(Default, Clone, Debug, PartialEq)]
pub enum XMLType<'render_pass, ImageElementData, CustomEvent: FromStr+Clone+PartialEq>{
    Bool(bool),
    Numeric(f32),
    Text(&'render_pass str),
    Image(&'render_pass ImageElementData),
    Color([f32;4]),
    ListLength(usize),
    Event(CustomEvent),
    #[default]
    None,
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum LayoutCommandType<CustomEvent: FromStr+Clone+PartialEq>{
    FlowControl(FlowControlCommand<CustomEvent>),
    ElementConfig(ConfigCommand),
    TextConfig(TextConfigCommand),
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum FlowControlCommand<CustomEvent: FromStr+Clone+PartialEq>{
    ElementOpened(Option<String>),
    ElementClosed,

    ConfigOpened(Option<String>),
    ConfigClosed,

    TextConfigOpened(Option<String>),
    TextConfigClosed,
    
    ListOpened(String),
    ListClosed,
    ListMember(String),

    CallOpened(String),
    CallClosed,

    Get(String, String),

    SetBool(String, bool),
    SetNumeric(String, f32),
    SetText(String, String),
    SetImage(String),
    SetColor(String, Color),
    SetListLength(String, usize),

    IfOpened(String),
    IfClosed,

    HoveredOpened,
    HoveredClosed,

    ClickedOpened(Option<CustomEvent>),
    ClickedClosed,
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum ConfigCommand{
    Id(String),
    GrowAll,
    GrowX,
    GrowXmin(f32),
    GrowXminmax(f32, f32),
    GrowY,
    GrowYmin(f32),
    GrowYminmax(f32, f32),
    FitX,
    FitXmin(f32),
    FitXminmax(f32, f32),
    FitY,
    FitYmin(f32),
    FitYminmax(f32, f32),
    FixedX(f32),
    FixedY(f32),
    PercentX(f32),
    PercentY(f32),
    PaddingAll(u16),
    PaddingTop(u16),
    PaddingBottom(u16),
    PaddingLeft(u16),
    PaddingRight(u16),
    ChildGap(u16),
    DirectionTTB,
    DirectionLTR,
    ChildAlignmentXLeft,
    ChildAlignmentXRight,
    ChildAlignmentXCenter,
    ChildAlignmentYTop,
    ChildAlignmentYCenter,
    ChildAlignmentYBottom,
    Color(Color),
    RadiusAll(f32),
    RadiusTopLeft(f32),
    RadiusTopRight(f32),
    RadiusBottomRight(f32),
    RadiusBottomLeft(f32),
    BorderAll(f32, Color),
    BorderTop(f32, Option<Color>),
    BorderLeft(f32, Option<Color>),
    BorderBottom(f32, Option<Color>),
    BorderRight(f32, Option<Color>),
    BorderBetweenChildren(f32, Option<Color>),
    Scroll(bool, bool),

    // todo:
    // floating elements
    // images
    // custom elements
    // custom layouts
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum TextConfigCommand{
    DefaultText(String),
    FontId(u16),
    AlignRight,
    AlignLeft,
    AlignCenter,
    LineHeight(u16),
    FontSize(u16),
    Content(String),
    DynamicContent(String),
    Color(Color),
}

#[derive(Default)]
enum ParsingMode{
    #[default]
    Normal,
    Fragment,
    ElementConfig,
    Textconfig,
}

#[allow(non_camel_case_types)]
#[derive(EnumString, Debug)]
enum SizeType{
    grow,
    fit,
    percent,
    fixed
}

#[allow(non_camel_case_types)]
#[derive(EnumString, PartialEq)]
enum LayoutData {
    left,
    right,
    center,
    top,
    bottom,
}

#[derive(Debug, Display)]
pub enum ParserError{
    UnNamedFragment,
    FragmentCallUnNamed,
    FileNotAccessable,
    ReaderError,
    UnknownTag(Vec<u8>),
}

trait Cdata {
    fn cdata(&self, value: &str) -> Option<String>;
}

impl Cdata for BytesStart<'_>{
    fn cdata(&self, value: &str) -> Option<String> {
        let optional_attr = match self.try_get_attribute(value) {
            Err(_) => return None,
            Ok(attr) => attr
        };
        let attr = match optional_attr {
            None => return None,
            Some(attribute) => attribute
        };
        let maybe_string = match attr.decode_and_unescape_value(Decoder {}) {
            Ok(a) => a,
            Err(_) => return None
        };
        Some(maybe_string.to_string())
    }
}

pub trait Get<ImageElementData, CustomEvent: FromStr+Clone+PartialEq>{
    #[allow(unused_variables)]
    fn get<'render_pass, 'application>(&'application self, name: &str) -> Option<XMLType::<'render_pass, ImageElementData, CustomEvent>> where 'application: 'render_pass{
        None
    }
    #[allow(unused_variables)]
    fn get_list_member<'render_pass, 'application>(&'application self, list_name: &str, list_index: usize, list_member: &str) -> Option<XMLType::<'render_pass, ImageElementData, CustomEvent>> where 'application: 'render_pass{
        None
    }
}

fn set_sizing_command<'a, UserEvents: FromStr+Clone+PartialEq>(bytes_start: &'a mut BytesStart, parser: &mut Parser<UserEvents>, horizontal: bool){
    let size_type = match bytes_start.cdata("type") {
        None => None,
        Some(size_type_string) => {
            match SizeType::from_str(&size_type_string) {
                Err(_) => None,
                Ok(size_type) => Some(size_type)
            }
        }
    };

    match size_type {
        None => {
            match horizontal {
                true => parser.push(LayoutCommandType::ElementConfig(ConfigCommand::FitX)),
                false => parser.push(LayoutCommandType::ElementConfig(ConfigCommand::FitY)),
            }
            return;
        }
        Some(size_type) => {
            match size_type {
                SizeType::fit => {
                    if let Some(min) = bytes_start.cdata("min") {
                        if let Ok(min) = f32::from_str(&min){
                            if let Some(max) = bytes_start.cdata("max") {
                                if let Ok(max) = f32::from_str(&max) {
                                    match horizontal {
                                        true => parser.push(LayoutCommandType::ElementConfig(ConfigCommand::FitXminmax(min, max))),
                                        false => parser.push(LayoutCommandType::ElementConfig(ConfigCommand::FitYminmax(min, max))),
                                    }
                                    return;
                                }
                            }
                            match horizontal {
                                true => parser.push(LayoutCommandType::ElementConfig(ConfigCommand::FitXmin(min))),
                                false => parser.push(LayoutCommandType::ElementConfig(ConfigCommand::FitYmin(min))),
                            }
                            return;
                        }
                    }
                    match horizontal {
                        true => parser.push(LayoutCommandType::ElementConfig(ConfigCommand::FitX)),
                        false => parser.push(LayoutCommandType::ElementConfig(ConfigCommand::FitY)),
                    }
                    return;
                }
                SizeType::grow => {
                    if let Some(min) = bytes_start.cdata("min") {
                        if let Ok(min) = f32::from_str(&min){
                            if let Some(max) = bytes_start.cdata("max") {
                                if let Ok(max) = f32::from_str(&max) {
                                    match horizontal {
                                        true => parser.push(LayoutCommandType::ElementConfig(ConfigCommand::GrowXminmax(min, max))),
                                        false => parser.push(LayoutCommandType::ElementConfig(ConfigCommand::GrowYminmax(min, max))),
                                    }
                                    return;
                                }
                            }
                            match horizontal {
                                true => parser.push(LayoutCommandType::ElementConfig(ConfigCommand::GrowXmin(min))),
                                false => parser.push(LayoutCommandType::ElementConfig(ConfigCommand::GrowYmin(min))),
                            }
                            return;
                        }
                    }
                    match horizontal {
                        true => parser.push(LayoutCommandType::ElementConfig(ConfigCommand::GrowX)),
                        false => parser.push(LayoutCommandType::ElementConfig(ConfigCommand::GrowY)),
                    }
                    return;
                }
                SizeType::fixed => {
                    if let Some(at) = bytes_start.cdata("at") {
                        if let Ok(at) = f32::from_str(&at) {
                            match horizontal {
                                true => parser.push(LayoutCommandType::ElementConfig(ConfigCommand::FixedX(at))),
                                false => parser.push(LayoutCommandType::ElementConfig(ConfigCommand::FixedY(at))),
                            }
                            return;
                        }
                    }
                    match horizontal {
                        true => parser.push(LayoutCommandType::ElementConfig(ConfigCommand::FitX)),
                        false => parser.push(LayoutCommandType::ElementConfig(ConfigCommand::FitY)),
                    }
                    return;
                }
                SizeType::percent => {
                    if let Some(at) = bytes_start.cdata("at") {
                        if let Ok(at) = f32::from_str(&at) {
                            match horizontal {
                                true => parser.push(LayoutCommandType::ElementConfig(ConfigCommand::PercentX(at))),
                                false => parser.push(LayoutCommandType::ElementConfig(ConfigCommand::PercentY(at))),
                            }
                            return;
                        }
                    }
                    match horizontal {
                        true => parser.push(LayoutCommandType::ElementConfig(ConfigCommand::FitX)),
                        false => parser.push(LayoutCommandType::ElementConfig(ConfigCommand::FitY)),
                    }
                    return;
                }
            }
        }
    }
}

#[derive(Default)]
pub struct Parser<UserEvents: FromStr+Clone+PartialEq>{
    mode: ParsingMode,

    current_page: Vec<LayoutCommandType<UserEvents>>,
    current_page_name: String,
    pages: HashMap<String, Vec<LayoutCommandType<UserEvents>>>,

    current_fragment: Vec<LayoutCommandType<UserEvents>>,
    fragment_name: String,
    fragments: HashMap<String, Vec<LayoutCommandType<UserEvents>>>,

    nesting_level: i32,
    xml_nesting_stack: Vec<i32>,

    text_opened: bool,
    text_content: Option<String>
}

impl<UserEvents: FromStr+Clone+PartialEq> Parser<UserEvents>{
    pub fn add_page(&mut self, xml_string: &str) -> Result<(), ParserError>{
        let mut reader = Reader::from_str(xml_string);
        reader.config_mut().trim_text(true);
        let mut buf = Vec::<u8>::new();

        self.push(LayoutCommandType::TextConfig(TextConfigCommand::DefaultText("hello".to_string())));

        loop {
            match reader.read_event_into(&mut buf) {
                Err(e) => {
                    println!("Reader Error: {:?}", e);
                    return Err(ParserError::ReaderError)
                },
                Ok(Event::Eof) => break,
                Ok(Event::Start(e)) => {
                    self.nest();
                    match e.name().as_ref() {
                        b"fragment" => {
                            match e.cdata("name") {
                                None => return Err(ParserError::UnNamedFragment),
                                Some(fragment_name) => self.open_fragment(fragment_name),
                            }
                        }
                        b"call" => {
                            match e.cdata("name") {
                                None => return Err(ParserError::FragmentCallUnNamed),
                                Some(fragment_name) => self.push(ConfigCommand::CallOpened(fragment_name.clone()))
                            }
                        }
                        b"page" => {
                            if let Some(_name) = e.cdata("name") {
                                //page_name = name;
                            }
                        }
                        b"element" => {
                            if let Some(id) = e.cdata("if") {
                                self.push_nest(ConfigCommand::IfOpened(id));
                            }
                            self.push(ConfigCommand::ElementOpened(e.cdata("id")));
                        }
                        b"layout" =>    self.push(ConfigCommand::ConfigOpened),
                        b"text" =>      self.push(ConfigCommand::TextOpened),
                        b"content" =>   self.start_text_content(),
                        b"hovered" =>   self.push(ConfigCommand::HoveredOpened),
                        b"list" => {
                            if let Some(source) = e.cdata("src") {
                                self.push(ConfigCommand::ListOpened(source));
                            }
                        }
                        b"clicked" => {
                            match e.cdata("emit") {
                                None => self.push(ConfigCommand::ClickedOpened(None)),
                                Some(event) => self.push(ConfigCommand::ClickedOpened(
                                    match CustomEvent::from_str(&event) {
                                        Err(_) => {
                                            println!("{:?}", event);
                                            None
                                        }
                                        Ok(event) => Some(event)
                                    }
                                )),
                            }
                        }
                        other => return Err(ParserError::UnknownTag(other.to_owned()))
                    }
                }
                Ok(Event::End(e)) => {
                    self.denest();
                    match e.name().as_ref() {
                        b"fragment" => self.close_fragment(),
                        b"call" => self.push(ConfigCommand::CallClosed),
                        b"page" => (),
                        b"element" => {
                            self.push(ConfigCommand::ElementClosed);
                            self.try_pop_nest();
                        }
                        b"layout" => self.push(ConfigCommand::ConfigClosed),
                        b"text" => self.push(ConfigCommand::TextClosed),
                        b"content" => self.close_text_content(),
                        b"hovered" => self.push(ConfigCommand::HoveredClosed),
                        b"clicked" => self.push(ConfigCommand::ClickedClosed),
                        b"list" => self.push(ConfigCommand::ListClosed),
                        other => return Err(ParserError::UnknownTag(other.to_owned()))
                    }
                }
                Ok(Event::Empty(mut e)) => {
                    match e.name().as_ref() {
                        b"id" => {
                            if let Some(id) = e.cdata("is") {
                                self.push(ConfigCommand::Id(id));
                            }
                        }
                        b"grow" => self.push(ConfigCommand::GrowAll),
                        b"width" => set_sizing_command(&mut e, &mut self, true),
                        b"height" => set_sizing_command(&mut e, &mut self, false),
                        b"padding" => {
                            if let Some(num) = e.cdata("all") {
                                if let Ok(num) = u16::from_str(&num) {
                                    self.push(ConfigCommand::PaddingAll(num));
                                }
                            }
                            if let Some(num) = e.cdata("top") {
                                if let Ok(num) = u16::from_str(&num) {
                                    self.push(ConfigCommand::PaddingTop(num));
                                }
                            }
                            if let Some(num) = e.cdata("bottom") {
                                if let Ok(num) = u16::from_str(&num) {
                                    self.push(ConfigCommand::PaddingBottom(num));
                                }
                            }
                            if let Some(num) = e.cdata("Left") {
                                if let Ok(num) = u16::from_str(&num) {
                                    self.push(ConfigCommand::PaddingLeft(num));
                                }
                            }
                            if let Some(num) = e.cdata("Right") {
                                if let Ok(num) = u16::from_str(&num) {
                                    self.push(ConfigCommand::PaddingRight(num));
                                }
                            }
                        }
                        b"direction" => {
                            if let Some(direction) = e.cdata("is") {
                                if &direction == "ttb" {
                                    self.push(ConfigCommand::DirectionTTB);
                                }
                                else {
                                    self.push(ConfigCommand::DirectionLTR);
                                }
                            }
                        }
                        b"align-children" => {
                            if let Some(alignment_x) = e.cdata("x") {
                                match LayoutData::from_str(&alignment_x) {
                                    Err(_) => {}
                                    Ok(layout_data) if layout_data == LayoutData::left => {
                                        self.push(ConfigCommand::ChildAlignmentXLeft);
                                    }
                                    Ok(layout_data) if layout_data == LayoutData::right => {
                                        self.push(ConfigCommand::ChildAlignmentXRight);
                                    }
                                    Ok(layout_data) if layout_data == LayoutData::center => {
                                        self.push(ConfigCommand::ChildAlignmentXCenter);
                                    }
                                    Ok(_) => {}
                                }
                            }
    
                            if let Some(alignment_y) = e.cdata("y") {
                                match LayoutData::from_str(&alignment_y) {
                                    Err(_) => {}
                                    Ok(layout_data) if layout_data == LayoutData::top => {
                                        self.push(ConfigCommand::ChildAlignmentYTop);
                                    }
                                    Ok(layout_data) if layout_data == LayoutData::bottom => {
                                        self.push(ConfigCommand::ChildAlignmentYBottom);
                                    }
                                    Ok(layout_data) if layout_data == LayoutData::center => {
                                        self.push(ConfigCommand::ChildAlignmentYCenter);
                                    }
                                    Ok(_) => {}
                                }
                            }
                        }
                        b"child-gap" => {
                            if let Some(is) = e.cdata("is") {
                                if let Ok(is) = u16::from_str(&is){
                                    self.push(ConfigCommand::ChildGap(is));
                                }
                            }
                        }
                        b"color" => {
                            let color = if let Some(color) = e.cdata("is") {
                                if let Ok(color) = csscolorself::parse(&color) {
                                    let color = color.to_rgba8();
                                    let color = [color[0] as f32, color[1] as f32, color[2] as f32, color[3] as f32];
                                    color
                                }
                                else { [0.0;4] }
                            } else { [0.0;4] };
                            
                            self.push_color(color);
                        }
                        b"dyn-color" => {
                            let color = if let Some(color) = e.cdata("from") {
                                if let Some(color) = app.get(&color) {
                                    if let XMLType::Color(color) = color {
                                        color
                                    }
                                    else { [0.0;4] }
                                } else { [0.0;4] }
                            } else { [0.0;4] };
    
                            self.push_color(color);
                        }
                        b"radius" => {
                            if let Some(radius) = e.cdata("all") {
                                if let Ok(radius) = f32::from_str(&radius) {
                                    self.push(ConfigCommand::RadiusAll(radius));
                                }
                            }
                            if let Some(radius) = e.cdata("top-left") {
                                if let Ok(radius) = f32::from_str(&radius) {
                                    self.push(ConfigCommand::RadiusTopLeft(radius));
                                }
                            }
                            if let Some(radius) = e.cdata("top-right") {
                                if let Ok(radius) = f32::from_str(&radius) {
                                    self.push(ConfigCommand::RadiusTopRight(radius));
                                }
                            }
                            if let Some(radius) = e.cdata("bottom-left") {
                                if let Ok(radius) = f32::from_str(&radius) {
                                    self.push(ConfigCommand::RadiusBottomLeft(radius));
                                }
                            }
                            if let Some(radius) = e.cdata("bottom-left") {
                                if let Ok(radius) = f32::from_str(&radius) {
                                    self.push(ConfigCommand::RadiusBottomRight(radius));
                                }
                            }
                        }
                        b"border" => {
                            let color = if let Some(color) = e.cdata("color"){
                                if let Ok(color) = csscolorparser::parse(&color) {
                                    let color = color.to_rgba8();
                                    let color = [color[0] as f32, color[1] as f32, color[2] as f32, color[3] as f32];
                                    Some(color)
                                }
                                else {
                                    None
                                }
                            } else {None};
                            if let Some(num) = e.cdata("all") {
                                if let Ok(num) = f32::from_str(&num) {
                                    match color {
                                        None => self.push(ConfigCommand::BorderAll(num, [0.0;4])),
                                        Some(color) => self.push(ConfigCommand::BorderAll(num, color)),
                                    }
                                }
                            }
                            if let Some(num) = e.cdata("top") {
                                if let Ok(num) = f32::from_str(&num) {
                                    self.push(ConfigCommand::BorderTop(num, color));
                                }
                            }
                            if let Some(num) = e.cdata("bottom") {
                                if let Ok(num) = f32::from_str(&num) {
                                    self.push(ConfigCommand::BorderBottom(num, color));
                                }
                            }
                            if let Some(num) = e.cdata("left") {
                                if let Ok(num) = f32::from_str(&num) {
                                    self.push(ConfigCommand::BorderLeft(num, color));
                                }
                            }
                            if let Some(num) = e.cdata("right") {
                                if let Ok(num) = f32::from_str(&num) {
                                    self.push(ConfigCommand::BorderRight(num, color));
                                }
                            }
                            if let Some(num) = e.cdata("between-children") {
                                if let Ok(num) = f32::from_str(&num) {
                                    self.push(ConfigCommand::BorderRight(num, color));
                                }
                            }
                        }
                        b"font-id" => {
                            if let Some(is) = e.cdata("is") {
                                if let Ok(is) = u16::from_str(&is){
                                    self.push(ConfigCommand::FontId(is));
                                }
                            }
                        }
                        b"text-align-left" => self.push(ConfigCommand::TextAlignLeft),
                        b"text-align-right" => self.push(ConfigCommand::TextAlignRight),
                        b"text-align-center" => self.push(ConfigCommand::TextAlignCenter),
                        b"font-size" => {
                            if let Some(is) = e.cdata("is") {
                                if let Ok(is) = u16::from_str(&is){
                                    self.push(ConfigCommand::FontSize(is));
                                }
                            }
                        }
                        b"line-height" => {
                            if let Some(is) = e.cdata("is") {
                                if let Ok(is) = u16::from_str(&is){
                                    self.push(ConfigCommand::TextLineHeight(is));
                                }
                            }
                        }
                        b"dyn-content" => {
                            if let Some(tag) = e.cdata("from") {
                                self.push(ConfigCommand::DynamicTextContent(tag));
                            }
                        }
                        b"get" => {
                            if let Some(local) = e.cdata("local") {
                                if let Some(name) = e.cdata("from") {
                                    self.push(ConfigCommand::Get(local, name));
                                }
                            }
                        }
                        b"set" => {
                            if let Some(local) = e.cdata("local") {
                                if let Some(value) = e.cdata("bool") {
                                    if let Ok(value) = bool::from_str(&value) {
                                        self.push(ConfigCommand::SetBool(local.clone(), value));
                                    }
                                }
                                if let Some(value) = e.cdata("numeric") {
                                    if let Ok(value) = f32::from_str(&value) {
                                        self.push(ConfigCommand::SetNumeric(local.clone(), value));
                                    }
                                }
                                if let Some(value) = e.cdata("text") {
                                    self.push(ConfigCommand::SetText(local.clone(), value));
                                }
                                // if let Some(value) = e.cdata("image") {
                                //     if let Ok(value) = f32::from_str(&value) {
                                //         self.push(LayoutCommands::Set(local.clone(), XMLType::Numeric(value)));
                                //     }
                                // }
                                if let Some(value) = e.cdata("color") {
                                    let color = if let Ok(color) = csscolorparser::parse(&value) {
                                            let color = color.to_rgba8();
                                            let color = [color[0] as f32, color[1] as f32, color[2] as f32, color[3] as f32];
                                            color
                                    } else { [0.0;4] };
                                    self.push(ConfigCommand::SetColor(local.clone(), color));
                                }
                            }
                        }
                        b"list-member" => {
                            if let Some(name) = e.cdata("name") {
                                self.push(ConfigCommand::ListMember(name));
                            }
                        }
                        b"scroll" => {
                            let vertical = match e.cdata("vertical") {
                                None => false,
                                Some(value) => bool::from_str(&value).unwrap()
                            };
                            let horizontal = match e.cdata("horizontal") {
                                None => false,
                                Some(value) => bool::from_str(&value).unwrap()
                            };
    
                            self.push(ConfigCommand::Scroll(vertical, horizontal));
                        }
                        
                        other => return Err(ParserError::UnknownTag(other.to_owned()))
                    }
                }
                Ok(Event::Text(e)) => self.receive_text_content(e.unescape().unwrap().to_string()),
                _ => ()
            }
        }
        Ok(())
    }
    fn push(&mut self, tag: LayoutCommandType<UserEvents>){
        if let LayoutCommandType::FlowControl(command) = &tag {
            match command {
                FlowControlCommand::TextConfigOpened(_) => self.text_opened = true,
                FlowControlCommand::TextConfigClosed => self.text_opened = false,
                _ => ()
            }
        }
        match self.mode {
            ParsingMode::Fragment => self.current_fragment.push(tag),
            ParsingMode::Normal => self.current_page.push(tag),
            _ => todo!()
        }
    }
    fn push_nest(&mut self, tag: LayoutCommandType<UserEvents>){
        self.push(tag);
        self.xml_nesting_stack.push(self.nesting_level);
    }
    fn try_pop_nest(&mut self){
        match self.xml_nesting_stack.last() {
            None => {}
            Some(saved_nesting_level) => {
                if self.nesting_level < *saved_nesting_level {
                    self.xml_nesting_stack.pop();
                    self.push(LayoutCommandType::FlowControl(FlowControlCommand::IfClosed));
                }
            }
        }
    }
    fn nest(&mut self){
        self.nesting_level += 1;
    }
    fn denest(&mut self){
        self.nesting_level -= 1;
    }
    fn start_text_content(&mut self){
        self.text_content = None;
    }
    fn receive_text_content(&mut self, content: String){
        if self.text_opened {
            self.text_content = Some(content);
        }
    }
    fn close_text_content(&mut self){
        let content = self.text_content.take().unwrap();
        self.push(LayoutCommandType::TextConfig(TextConfigCommand::Content(content)));
    }
    fn open_fragment(&mut self, name: String){
        self.fragment_name = name;
        self.current_fragment.clear();
        self.mode = ParsingMode::Fragment;
    }
    fn close_fragment(&mut self){
        let new_fragment = self.current_fragment.clone();
        self.fragments.insert(self.fragment_name.clone(), new_fragment);
        self.mode = ParsingMode::Normal;
    }

    pub fn set_page<'render_pass, ImageElementData: Debug+Default, CustomElementData: Debug+Default, UserApp: Get<ImageElementData, UserEvents>>(
        page: &str,
        clicked: bool,
        events: &mut Vec<UserEvents>,
        layout_engine: &mut LayoutEngine<ImageElementData, CustomElementData, UserEvents>,
        user_app: &UserApp
    ){}
}

pub fn parse_xml<ImageElementData, CustomEvent: FromStr+Clone+PartialEq+Default+Debug, UserApp: Get<ImageElementData, CustomEvent>>(file: &str, app: &mut UserApp)
    where <CustomEvent as FromStr>::Err: Debug
{
}

fn set_layout<'render_pass, ImageElementData: Debug+Default, CustomElementData: Debug+Default, UserEvents: FromStr+Clone+PartialEq+Debug, UserApp: Get<ImageElementData, UserEvents>>(
    clicked: bool,
    events: &mut Vec<UserEvents>,
    command_stack: &Vec<LayoutCommandType<UserEvents>>,
    fragments: &HashMap<String, Vec<LayoutCommandType<UserEvents>>>,
    layout_engine: &mut LayoutEngine<ImageElementData, CustomElementData, UserEvents>,
    user_app: &UserApp,
    locals: Option<&HashMap<String, XMLType<ImageElementData, UserEvents>>>
){
    let mut config = None::<ElementConfiguration>;
    let mut selected_fragment = None::<&Vec<LayoutCommandType<UserEvents>>>;
    let mut local_call_stack = HashMap::<String, XMLType<ImageElementData, UserEvents>>::new();
    let mut text_config = None::<TextConfig>;
    let mut text_content = None::<&String>;
    let mut dynamic_text_content = None::<&'render_pass str>;
    let mut nesting_level = 0;
    let mut skip_level = -1;

    let mut list_stack = Vec::<LayoutCommandType<UserEvents>>::new();
    let mut list_members = Vec::<LayoutCommandType<UserEvents>>::new();
    let mut list_opened = false;
    let mut list_source = String::new();

    for xml_command in command_stack.iter() {
        if list_opened {
            match xml_command {
                ConfigCommand::ListClosed => {
                    list_opened = false;
                }
                ConfigCommand::ListMember(_) => {
                    list_members.push(xml_command.clone())
                }
                other => {
                    list_stack.push(other.clone());
                    continue;
                }
            }
        }

        match xml_command {
            ConfigCommand::IfOpened(tag) => {
                if skip_level == -1 && !get_bool(&tag, user_app) {
                    skip_level = nesting_level;
                }

                nesting_level += 1;
            }
            ConfigCommand::IfClosed => {
                nesting_level -= 1;

                if skip_level == nesting_level {
                    skip_level = -1;
                }
            }
            ConfigCommand::HoveredOpened => {
                if skip_level == -1 && !layout_engine.hovered() {
                    skip_level = nesting_level;
                }

                nesting_level += 1;
            }
            ConfigCommand::HoveredClosed => {
                nesting_level -= 1;

                if skip_level == nesting_level {
                    skip_level = -1;
                }
            }
            ConfigCommand::ClickedOpened(event) => {
                if skip_level == -1 && (!clicked || !layout_engine.hovered()) {
                    skip_level = nesting_level;
                }
                else {
                    if let Some(event) = event {
                        events.push(event.clone());
                    }
                }

                nesting_level += 1;
            }
            ConfigCommand::ClickedClosed => {
                nesting_level -= 1;

                if skip_level == nesting_level {
                    skip_level = -1;
                }
            }
            ConfigCommand::ElementOpened(_id)=> {
                nesting_level += 1;

                if skip_level == -1 {
                    layout_engine.open();
                }
            }
            ConfigCommand::ElementClosed=> {
                nesting_level -= 1;

                if skip_level == -1 {
                    layout_engine.close();
                }
            }
            ConfigCommand::ConfigOpened  => {
                nesting_level += 1;

                if skip_level == -1 {
                    match config.is_some() {
                        false => config = Some(layout_engine.start_config()),
                        true => {} 
                    }
                }
            }
            ConfigCommand::ConfigClosed  => {
                nesting_level -= 1;

                if skip_level == -1 {
                    match config.is_some() {
                        false => panic!("invalid xml stack"),
                        true => {
                            let final_config = config.take().unwrap();
                            //println!("final config:{:?}", final_config);
                            layout_engine.end_config(final_config);
                        },
                    }
                }
            }
            
            ConfigCommand::CallOpened(fragment_name)  => {
                if let Some(fragment) = fragments.get(fragment_name) {
                    selected_fragment = Some(fragment);
                    local_call_stack.clear();
                }
            }
            ConfigCommand::CallClosed  => {
                if selected_fragment.is_some() {
                    if local_call_stack.len() > 0 {
                        set_layout(clicked, events, selected_fragment.take().unwrap(), fragments, layout_engine, user_app, Some(&local_call_stack));
                    }
                    else {
                        set_layout(clicked, events, selected_fragment.take().unwrap(), fragments, layout_engine, user_app, None);
                    }
                }
            }

            ConfigCommand::ListOpened(source)  => {
                list_source = source.to_string();
                list_stack.clear();
                list_members.clear();
                list_opened = true;
            }
            ConfigCommand::ListClosed  => {
                let list_length = user_app.get(&list_source);
                if let Some(source) = list_length {
                    if let XMLType::ListLength(length) = source {
                        for i in 0..length {
                            local_call_stack.clear();
                            for member in list_members.iter() {
                                if let ConfigCommand::ListMember(name) = member {
                                    let value = user_app.get_list_member(&list_source, i, &name);
                                    if let Some(value) = value {
                                        local_call_stack.insert(name.to_string(), value);
                                    }
                                }
                            }
                            set_layout(clicked, events,  &list_stack, fragments, layout_engine, user_app, Some(&local_call_stack));
                        }
                    }
                }
            }
            
            ConfigCommand::Get(local, name)  => {
                if let Some(value) = user_app.get(name) {
                    local_call_stack.insert(local.to_string(), value);
                }
            }
            // todo other sets
            ConfigCommand::SetBool(local, value)  => {
                local_call_stack.insert(local.to_string(), XMLType::Bool(*value));
            }
            ConfigCommand::SetNumeric(local, value)  => {
                local_call_stack.insert(local.to_string(), XMLType::Numeric(*value));
            }
            ConfigCommand::SetText(local, value)  => {
                local_call_stack.insert(local.to_string(), XMLType::Text(&value.as_str()));
            }

            config_command => {
                if skip_level == -1 {
                    let open_config = config.as_mut().unwrap();

                    match config_command {
                        ConfigCommand::FitX  => open_config.x_fit().parse(),
                        ConfigCommand::FitXmin(min)  => open_config.x_fit_min(*min).parse(),
                        ConfigCommand::FitXminmax(min, max)  => open_config.x_fit_min_max(*min, *max).parse(),
                        ConfigCommand::FitY  => open_config.y_fit().parse(),
                        ConfigCommand::FitYmin(min)  => open_config.y_fit_min(*min).parse(),
                        ConfigCommand::FitYminmax(min, max)  => open_config.y_fit_min_max(*min, *max).parse(),
                        ConfigCommand::GrowX  => open_config.x_grow().parse(),
                        ConfigCommand::GrowXmin(min)  => open_config.x_grow_min(*min).parse(),
                        ConfigCommand::GrowXminmax(min, max)  => open_config.x_grow_min_max(*min, *max).parse(),
                        ConfigCommand::GrowY  => open_config.y_grow().parse(),
                        ConfigCommand::GrowYmin(min)  => open_config.y_grow_min(*min).parse(),
                        ConfigCommand::GrowYminmax(min, max)  => open_config.y_grow_min_max(*min, *max).parse(),
                        ConfigCommand::FixedX(x)  => open_config.x_fixed(*x).parse(),
                        ConfigCommand::FixedY(y)  => open_config.y_fixed(*y).parse(),
                        ConfigCommand::PercentX(size)  => open_config.x_percent(*size).parse(),
                        ConfigCommand::PercentY(size)  => open_config.y_percent(*size).parse(),
                        ConfigCommand::GrowAll  => open_config.grow_all().parse(),
                        ConfigCommand::PaddingAll(padding)  => open_config.padding_all(*padding).parse(),
                        ConfigCommand::PaddingTop(padding)  => open_config.padding_top(*padding).parse(),
                        ConfigCommand::PaddingBottom(padding)  => open_config.padding_bottom(*padding).parse(),
                        ConfigCommand::PaddingLeft(padding)  => open_config.padding_left(*padding).parse(),
                        ConfigCommand::PaddingRight(padding)  => open_config.padding_right(*padding).parse(),
                        ConfigCommand::DirectionTTB  => open_config.direction(true).parse(),
                        ConfigCommand::DirectionLTR  => open_config.direction(false).parse(),
                        ConfigCommand::Id(label)  => open_config.id(&label).parse(),
                        ConfigCommand::ChildGap(gap)  => open_config.child_gap(*gap).parse(),
                        ConfigCommand::ChildAlignmentXLeft  => open_config.align_children_x_left().parse(),
                        ConfigCommand::ChildAlignmentXRight  => open_config.align_children_x_right().parse(),
                        ConfigCommand::ChildAlignmentXCenter  => open_config.align_children_x_center().parse(),
                        ConfigCommand::ChildAlignmentYTop  => open_config.align_children_y_top().parse(),
                        ConfigCommand::ChildAlignmentYCenter  => open_config.align_children_y_center().parse(),
                        ConfigCommand::ChildAlignmentYBottom  => open_config.align_children_y_bottom().parse(),
                        ConfigCommand::Color(color)  => todo!(),
                        ConfigCommand::RadiusAll(radius)  => open_config.radius_all(*radius).parse(),
                        ConfigCommand::RadiusTopLeft(radius)  => open_config.radius_top_left(*radius).parse(),
                        ConfigCommand::RadiusTopRight(radius)  => open_config.radius_bottom_right(*radius).parse(),
                        ConfigCommand::RadiusBottomRight(radius)  => open_config.radius_bottom_right(*radius).parse(),
                        ConfigCommand::RadiusBottomLeft(radius)  => open_config.radius_bottom_left(*radius).parse(),
                        ConfigCommand::BorderAll(border, color)  => todo!(),
                        ConfigCommand::BorderTop(border, color)  => todo!(),
                        ConfigCommand::BorderBottom(border, color)  => todo!(),
                        ConfigCommand::BorderLeft(border, color)  => todo!(),
                        ConfigCommand::BorderRight(border, color)  => todo!(),
                        ConfigCommand::BorderBetweenChildren(border, color)  => todo!(),
                        ConfigCommand::TextOpened  => {
                            nesting_level += 1;
            
                            text_config = Some(TextConfig::default());
                        } 
                        ConfigCommand::TextClosed  => {
                            match text_config.is_some() {
                                false => panic!("invalid xml stack"),
                                true => {
                                    let final_text_config = text_config.take().unwrap();
                                    match text_content.is_some() {
                                        false => {
                                            match dynamic_text_content.is_some() {
                                                false => {
                                                    
                                                    layout_engine.add_text_element("", &final_text_config.end(), false);
                                                }
                                                true => {
                                                    let final_dyn_content = dynamic_text_content.take().unwrap();
                                                    layout_engine.add_text_element(&final_dyn_content, &final_text_config.end(), false);
                                                }
                                            }
                                        }
                                        true => {
                                            layout_engine.add_text_element(text_content.take().unwrap(), &final_text_config.end(), false);
                                        }
                                    }
                                    
                                },
                            }
            
                            nesting_level -= 1;
                        }
                        ConfigCommand::FontId(id)  => {
                            text_config.as_mut().unwrap().font_id(*id);
                        }
                        ConfigCommand::TextAlignLeft  => {
                            text_config.as_mut().unwrap().alignment(crate::text::TextAlignment::Left);
                        }
                        ConfigCommand::TextAlignRight  => {
                            text_config.as_mut().unwrap().alignment(crate::text::TextAlignment::Right);
                        }
                        ConfigCommand::TextAlignCenter  => {
                            text_config.as_mut().unwrap().alignment(crate::text::TextAlignment::Center);
                        }
                        ConfigCommand::TextLineHeight(lh)  => {
                            text_config.as_mut().unwrap().line_height(*lh);
                        }
                        ConfigCommand::FontSize(size)  => {
                            text_config.as_mut().unwrap().font_size(*size);
                        }
                        ConfigCommand::TextContent(content)  => {
                            text_content = Some(content);
                        }
                        ConfigCommand::DynamicTextContent(content)  => {
                            match locals.is_some() {
                                true => {
                                    match locals.as_ref().unwrap().get(content) {
                                        None => dynamic_text_content = Some(get_text(&content, user_app).unwrap()),
                                        Some(local) => {
                                            match local {
                                                XMLType::Text(text) => dynamic_text_content = Some(text),
                                                _ => dynamic_text_content = None
                                            }
                                        }
                                    }
                                },
                                false => dynamic_text_content = Some(get_text(&content, user_app).unwrap())
                            }
                        }
                        ConfigCommand::TextColor(color)  => {
                            text_config.as_mut().unwrap().color(color[0], color[1], color[2], color[3]);
                        }
            
                        ConfigCommand::Scroll(vertical, horizontal)  => {
                            config.as_mut().unwrap().scroll(*vertical, *horizontal);
                        }
                        other_command => {}
                    };
                }
            }
        }

        match xml_command {
            ConfigCommand::IfOpened(tag) => {
                if skip_level == -1 && !get_bool(&tag, user_app) {
                    skip_level = nesting_level;
                }

                nesting_level += 1;
            }
            ConfigCommand::IfClosed => {
                nesting_level -= 1;

                if skip_level == nesting_level {
                    skip_level = -1;
                }
            }
            ConfigCommand::HoveredOpened => {
                if skip_level == -1 && !layout_engine.hovered() {
                    skip_level = nesting_level;
                }

                nesting_level += 1;
            }
            ConfigCommand::HoveredClosed => {
                nesting_level -= 1;

                if skip_level == nesting_level {
                    skip_level = -1;
                }
            }
            ConfigCommand::ClickedOpened(event) => {
                if skip_level == -1 && (!clicked || !layout_engine.hovered()) {
                    skip_level = nesting_level;
                }
                else {
                    if let Some(event) = event {
                        events.push(event.clone());
                    }
                }

                nesting_level += 1;
            }
            ConfigCommand::ClickedClosed => {
                nesting_level -= 1;

                if skip_level == nesting_level {
                    skip_level = -1;
                }
            }
            ConfigCommand::ElementOpened(_id)=> {
                nesting_level += 1;

                if skip_level == -1 {
                    layout_engine.open();
                }
            }
            ConfigCommand::ElementClosed=> {
                nesting_level -= 1;

                if skip_level == -1 {
                    layout_engine.close();
                }
            }
            
            
            _other => {}//println!("unused layout command: {:}", other);}
        }
    }
}