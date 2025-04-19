use std::{collections::HashMap, fmt::Debug, fs, path::Path, str::FromStr};

use quick_xml::events::BytesStart;
use quick_xml::Decoder;
use strum_macros::{Display, EnumString};

use crate::{bindings::true_, ElementConfiguration, LayoutEngine, TextConfig};

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

#[derive(Clone, Debug, Display, Default, PartialEq)]
pub enum LayoutCommands<CustomEvent: FromStr+Clone+PartialEq>{
    #[default]
    None,
    DefaultText(String),

    ListOpened(String),
    ListClosed,
    ListMember(String),

    ElementOpened(Option<String>),
    ElementClosed,
    ConfigOpened,
    ConfigClosed,
    CallOpened(String),
    CallClosed,

    Get(String, String),

    SetBool(String, bool),
    SetNumeric(String, f32),
    SetText(String, String),
    SetImage(String),
    SetColor(String, [f32;4]),

    IfOpened(String),
    IfClosed,

    HoveredOpened,
    HoveredClosed,

    ClickedOpened(Option<CustomEvent>),
    ClickedClosed,

    FitX,
    FitXmin(f32),
    FitXminmax(f32, f32),
    FitY,
    FitYmin(f32),
    FitYminmax(f32, f32),

    GrowX,
    GrowXmin(f32),
    GrowXminmax(f32, f32),
    GrowY,
    GrowYmin(f32),
    GrowYminmax(f32, f32),

    FixedX(f32),
    FixedY(f32),

    PercentX(f32),
    PercentY(f32),

    GrowAll,

    PaddingAll(f32),
    PaddingTop(f32),
    PaddingBottom(f32),
    PaddingLeft(f32),
    PaddingRight(f32),

    DirectionTTB,
    DirectionLTR,

    Id(String),

    ChildGap(u16),

    ChildAlignmentXLeft,
    ChildAlignmentXRight,
    ChildAlignmentXCenter,

    ChildAlignmentYTop,
    ChildAlignmentYCenter,
    ChildAlignmentYBottom,

    Color([f32;4]),

    RadiusAll(f32),
    RadiusTopLeft(f32),
    RadiusTopRight(f32),
    RadiusBottomRight(f32),
    RadiusBottomLeft(f32),

    BorderAll(f32,[f32;4]),
    BorderTop(f32, Option<[f32;4]>),
    BorderLeft(f32, Option<[f32;4]>),
    BorderBottom(f32, Option<[f32;4]>),
    BorderRight(f32, Option<[f32;4]>),
    BorderBetweenChildren(f32, Option<[f32;4]>),

    TextOpened,
    TextClosed,
    FontId(u16),
    TextAlignRight,
    TextAlignLeft,
    TextAlignCenter,
    TextLineHeight(u16),
    FontSize(u16),
    TextContent(String),
    DynamicTextContent(String),
    TextColor([f32;4]),

    Scroll(bool, bool),


    // todo:
    // images
    // floating elements
    // custom elements
    // scolling elements
    // custom layouts
    //
    // clicked
}

#[derive(Default)]
enum ParsingMode{
    #[default]
    Normal,
    Fragment
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
                true => parser.push(LayoutCommands::FitX),
                false => parser.push(LayoutCommands::FitY),
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
                                        true => parser.push(LayoutCommands::FitXminmax(min, max)),
                                        false => parser.push(LayoutCommands::FitYminmax(min, max)),
                                    }
                                    return;
                                }
                            }
                            match horizontal {
                                true => parser.push(LayoutCommands::FitXmin(min)),
                                false => parser.push(LayoutCommands::FitYmin(min)),
                            }
                            return;
                        }
                    }
                    match horizontal {
                        true => parser.push(LayoutCommands::FitX),
                        false => parser.push(LayoutCommands::FitY),
                    }
                    return;
                }
                SizeType::grow => {
                    if let Some(min) = bytes_start.cdata("min") {
                        if let Ok(min) = f32::from_str(&min){
                            if let Some(max) = bytes_start.cdata("max") {
                                if let Ok(max) = f32::from_str(&max) {
                                    match horizontal {
                                        true => parser.push(LayoutCommands::GrowXminmax(min, max)),
                                        false => parser.push(LayoutCommands::GrowYminmax(min, max)),
                                    }
                                    return;
                                }
                            }
                            match horizontal {
                                true => parser.push(LayoutCommands::GrowXmin(min)),
                                false => parser.push(LayoutCommands::GrowYmin(min)),
                            }
                            return;
                        }
                    }
                    match horizontal {
                        true => parser.push(LayoutCommands::GrowX),
                        false => parser.push(LayoutCommands::GrowY),
                    }
                    return;
                }
                SizeType::fixed => {
                    if let Some(at) = bytes_start.cdata("at") {
                        if let Ok(at) = f32::from_str(&at) {
                            match horizontal {
                                true => parser.push(LayoutCommands::FixedX(at)),
                                false => parser.push(LayoutCommands::FixedY(at)),
                            }
                            return;
                        }
                    }
                    match horizontal {
                        true => parser.push(LayoutCommands::FitX),
                        false => parser.push(LayoutCommands::FitY),
                    }
                    return;
                }
                SizeType::percent => {
                    if let Some(at) = bytes_start.cdata("at") {
                        if let Ok(at) = f32::from_str(&at) {
                            match horizontal {
                                true => parser.push(LayoutCommands::PercentX(at)),
                                false => parser.push(LayoutCommands::PercentY(at)),
                            }
                            return;
                        }
                    }
                    match horizontal {
                        true => parser.push(LayoutCommands::FitX),
                        false => parser.push(LayoutCommands::FitY),
                    }
                    return;
                }
            }
        }
    }
}

#[derive(Default)]
struct Parser<UserEvents: FromStr+Clone+PartialEq>{
    pub mode: ParsingMode,
    xml_stack: Vec<LayoutCommands<UserEvents>>,

    fragment: Vec<LayoutCommands<UserEvents>>,
    fragment_name: String,
    fragments: HashMap<String, Vec<LayoutCommands<UserEvents>>>,

    nesting_level: i32,
    xml_nesting_stack: Vec<i32>,

    text_opened: bool,
    text_content: Option<String>
}

impl<UserEvents: FromStr+Clone+PartialEq> Parser<UserEvents>{
    fn push(&mut self, tag: LayoutCommands<UserEvents>){
        if let LayoutCommands::TextOpened = tag {
            self.text_opened = true;
        }
        if tag == LayoutCommands::TextClosed {
            self.text_opened = false;
        }
        match self.mode {
            ParsingMode::Fragment => self.fragment.push(tag),
            ParsingMode::Normal => self.xml_stack.push(tag),
        }
    }
    fn push_color(&mut self, color: [f32;4]){
        if self.text_opened {
            self.push(LayoutCommands::TextColor(color));
        }
        else {
            self.push(LayoutCommands::Color(color));
        }
    }
    fn push_nest(&mut self, tag: LayoutCommands<UserEvents>){
        self.push(tag);
        self.xml_nesting_stack.push(self.nesting_level);
    }
    fn try_pop_nest(&mut self){
        match self.xml_nesting_stack.last() {
            None => {}
            Some(saved_nesting_level) => {
                if self.nesting_level < *saved_nesting_level {
                    self.xml_nesting_stack.pop();
                    self.push(LayoutCommands::IfClosed);
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
        self.push(LayoutCommands::TextContent(content));
    }
    fn open_fragment(&mut self, name: String){
        self.fragment_name = name;
        self.fragment.clear();
        self.mode = ParsingMode::Fragment;
    }
    fn close_fragment(&mut self){
        let new_fragment = self.fragment.clone();
        self.fragments.insert(self.fragment_name.clone(), new_fragment);
        self.mode = ParsingMode::Normal;
    }
    fn flush(self) -> (Vec<LayoutCommands<UserEvents>>, HashMap<String, Vec<LayoutCommands<UserEvents>>>) {
        (self.xml_stack, self.fragments)
    }
}

pub fn parse_xml<ImageElementData, CustomEvent: FromStr+Clone+PartialEq+Default+Debug, UserApp: Get<ImageElementData, CustomEvent>>(file: &str, app: &mut UserApp) -> Result<(Vec<LayoutCommands<CustomEvent>>, HashMap::<String, Vec<LayoutCommands<CustomEvent>>>), ParserError>
    where <CustomEvent as FromStr>::Err: Debug
{
    use quick_xml::events::Event;
    use quick_xml::reader::Reader;

    let path = Path::new(file);
    let file = match fs::read_to_string(path) {
        Ok(file) => file,
        Err(_) => return Err(ParserError::FileNotAccessable)
    };

    let mut reader = Reader::from_str(&file);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();

    let mut parser = Parser::<CustomEvent>::default();

    parser.push(LayoutCommands::DefaultText("hello".to_string()));

    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => {
                println!("Reader Error: {:?}", e);
                return Err(ParserError::ReaderError)
            },
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => {
                parser.nest();
                match e.name().as_ref() {
                    b"fragment" => {
                        match e.cdata("name") {
                            None => return Err(ParserError::UnNamedFragment),
                            Some(fragment_name) => parser.open_fragment(fragment_name),
                        }
                    }
                    b"call" => {
                        match e.cdata("name") {
                            None => return Err(ParserError::FragmentCallUnNamed),
                            Some(fragment_name) => parser.push(LayoutCommands::CallOpened(fragment_name.clone()))
                        }
                    }
                    b"page" => {
                        if let Some(_name) = e.cdata("name") {
                            //page_name = name;
                        }
                    }
                    b"element" => {
                        if let Some(id) = e.cdata("if") {
                            parser.push_nest(LayoutCommands::IfOpened(id));
                        }
                        parser.push(LayoutCommands::ElementOpened(e.cdata("id")));
                    }
                    b"layout" =>    parser.push(LayoutCommands::ConfigOpened),
                    b"text" =>      parser.push(LayoutCommands::TextOpened),
                    b"content" =>   parser.start_text_content(),
                    b"hovered" =>   parser.push(LayoutCommands::HoveredOpened),
                    b"list" => {
                        if let Some(source) = e.cdata("src") {
                            parser.push(LayoutCommands::ListOpened(source));
                        }
                    }
                    b"clicked" => {
                        match e.cdata("emit") {
                            None => parser.push(LayoutCommands::ClickedOpened(None)),
                            Some(event) => parser.push(LayoutCommands::ClickedOpened(
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
                parser.denest();
                match e.name().as_ref() {
                    b"fragment" => parser.close_fragment(),
                    b"call" => parser.push(LayoutCommands::CallClosed),
                    b"page" => (),
                    b"element" => {
                        parser.push(LayoutCommands::ElementClosed);
                        parser.try_pop_nest();
                    }
                    b"layout" => parser.push(LayoutCommands::ConfigClosed),
                    b"text" => parser.push(LayoutCommands::TextClosed),
                    b"content" => parser.close_text_content(),
                    b"hovered" => parser.push(LayoutCommands::HoveredClosed),
                    b"clicked" => parser.push(LayoutCommands::ClickedClosed),
                    b"list" => parser.push(LayoutCommands::ListClosed),
                    other => return Err(ParserError::UnknownTag(other.to_owned()))
                }
            }
            Ok(Event::Empty(mut e)) => {
                match e.name().as_ref() {
                    b"id" => {
                        if let Some(id) = e.cdata("is") {
                            parser.push(LayoutCommands::Id(id));
                        }
                    }
                    b"grow" => parser.push(LayoutCommands::GrowAll),
                    b"width" => set_sizing_command(&mut e, &mut parser, true),
                    b"height" => set_sizing_command(&mut e, &mut parser, false),
                    b"padding" => {
                        if let Some(num) = e.cdata("all") {
                            if let Ok(num) = f32::from_str(&num) {
                                parser.push(LayoutCommands::PaddingAll(num));
                            }
                        }
                        if let Some(num) = e.cdata("top") {
                            if let Ok(num) = f32::from_str(&num) {
                                parser.push(LayoutCommands::PaddingTop(num));
                            }
                        }
                        if let Some(num) = e.cdata("bottom") {
                            if let Ok(num) = f32::from_str(&num) {
                                parser.push(LayoutCommands::PaddingBottom(num));
                            }
                        }
                        if let Some(num) = e.cdata("Left") {
                            if let Ok(num) = f32::from_str(&num) {
                                parser.push(LayoutCommands::PaddingLeft(num));
                            }
                        }
                        if let Some(num) = e.cdata("Right") {
                            if let Ok(num) = f32::from_str(&num) {
                                parser.push(LayoutCommands::PaddingRight(num));
                            }
                        }
                    }
                    b"direction" => {
                        if let Some(direction) = e.cdata("is") {
                            if &direction == "ttb" {
                                parser.push(LayoutCommands::DirectionTTB);
                            }
                            else {
                                parser.push(LayoutCommands::DirectionLTR);
                            }
                        }
                    }
                    b"align-children" => {
                        if let Some(alignment_x) = e.cdata("x") {
                            match LayoutData::from_str(&alignment_x) {
                                Err(_) => {}
                                Ok(layout_data) if layout_data == LayoutData::left => {
                                    parser.push(LayoutCommands::ChildAlignmentXLeft);
                                }
                                Ok(layout_data) if layout_data == LayoutData::right => {
                                    parser.push(LayoutCommands::ChildAlignmentXRight);
                                }
                                Ok(layout_data) if layout_data == LayoutData::center => {
                                    parser.push(LayoutCommands::ChildAlignmentXCenter);
                                }
                                Ok(_) => {}
                            }
                        }

                        if let Some(alignment_y) = e.cdata("y") {
                            match LayoutData::from_str(&alignment_y) {
                                Err(_) => {}
                                Ok(layout_data) if layout_data == LayoutData::top => {
                                    parser.push(LayoutCommands::ChildAlignmentYTop);
                                }
                                Ok(layout_data) if layout_data == LayoutData::bottom => {
                                    parser.push(LayoutCommands::ChildAlignmentYBottom);
                                }
                                Ok(layout_data) if layout_data == LayoutData::center => {
                                    parser.push(LayoutCommands::ChildAlignmentYCenter);
                                }
                                Ok(_) => {}
                            }
                        }
                    }
                    b"child-gap" => {
                        if let Some(is) = e.cdata("is") {
                            if let Ok(is) = u16::from_str(&is){
                                parser.push(LayoutCommands::ChildGap(is));
                            }
                        }
                    }
                    b"color" => {
                        let color = if let Some(color) = e.cdata("is") {
                            if let Ok(color) = csscolorparser::parse(&color) {
                                let color = color.to_rgba8();
                                let color = [color[0] as f32, color[1] as f32, color[2] as f32, color[3] as f32];
                                color
                            }
                            else { [0.0;4] }
                        } else { [0.0;4] };
                        
                        parser.push_color(color);
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

                        parser.push_color(color);
                    }
                    b"radius" => {
                        if let Some(radius) = e.cdata("all") {
                            if let Ok(radius) = f32::from_str(&radius) {
                                parser.push(LayoutCommands::RadiusAll(radius));
                            }
                        }
                        if let Some(radius) = e.cdata("top-left") {
                            if let Ok(radius) = f32::from_str(&radius) {
                                parser.push(LayoutCommands::RadiusTopLeft(radius));
                            }
                        }
                        if let Some(radius) = e.cdata("top-right") {
                            if let Ok(radius) = f32::from_str(&radius) {
                                parser.push(LayoutCommands::RadiusTopRight(radius));
                            }
                        }
                        if let Some(radius) = e.cdata("bottom-left") {
                            if let Ok(radius) = f32::from_str(&radius) {
                                parser.push(LayoutCommands::RadiusBottomLeft(radius));
                            }
                        }
                        if let Some(radius) = e.cdata("bottom-left") {
                            if let Ok(radius) = f32::from_str(&radius) {
                                parser.push(LayoutCommands::RadiusBottomRight(radius));
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
                                    None => parser.push(LayoutCommands::BorderAll(num, [0.0;4])),
                                    Some(color) => parser.push(LayoutCommands::BorderAll(num, color)),
                                }
                            }
                        }
                        if let Some(num) = e.cdata("top") {
                            if let Ok(num) = f32::from_str(&num) {
                                parser.push(LayoutCommands::BorderTop(num, color));
                            }
                        }
                        if let Some(num) = e.cdata("bottom") {
                            if let Ok(num) = f32::from_str(&num) {
                                parser.push(LayoutCommands::BorderBottom(num, color));
                            }
                        }
                        if let Some(num) = e.cdata("left") {
                            if let Ok(num) = f32::from_str(&num) {
                                parser.push(LayoutCommands::BorderLeft(num, color));
                            }
                        }
                        if let Some(num) = e.cdata("right") {
                            if let Ok(num) = f32::from_str(&num) {
                                parser.push(LayoutCommands::BorderRight(num, color));
                            }
                        }
                        if let Some(num) = e.cdata("between-children") {
                            if let Ok(num) = f32::from_str(&num) {
                                parser.push(LayoutCommands::BorderRight(num, color));
                            }
                        }
                    }
                    b"font-id" => {
                        if let Some(is) = e.cdata("is") {
                            if let Ok(is) = u16::from_str(&is){
                                parser.push(LayoutCommands::FontId(is));
                            }
                        }
                    }
                    b"text-align-left" => parser.push(LayoutCommands::TextAlignLeft),
                    b"text-align-right" => parser.push(LayoutCommands::TextAlignRight),
                    b"text-align-center" => parser.push(LayoutCommands::TextAlignCenter),
                    b"font-size" => {
                        if let Some(is) = e.cdata("is") {
                            if let Ok(is) = u16::from_str(&is){
                                parser.push(LayoutCommands::FontSize(is));
                            }
                        }
                    }
                    b"line-height" => {
                        if let Some(is) = e.cdata("is") {
                            if let Ok(is) = u16::from_str(&is){
                                parser.push(LayoutCommands::TextLineHeight(is));
                            }
                        }
                    }
                    b"dyn-content" => {
                        if let Some(tag) = e.cdata("from") {
                            parser.push(LayoutCommands::DynamicTextContent(tag));
                        }
                    }
                    b"get" => {
                        if let Some(local) = e.cdata("local") {
                            if let Some(name) = e.cdata("from") {
                                parser.push(LayoutCommands::Get(local, name));
                            }
                        }
                    }
                    b"set" => {
                        if let Some(local) = e.cdata("local") {
                            if let Some(value) = e.cdata("bool") {
                                if let Ok(value) = bool::from_str(&value) {
                                    parser.push(LayoutCommands::SetBool(local.clone(), value));
                                }
                            }
                            if let Some(value) = e.cdata("numeric") {
                                if let Ok(value) = f32::from_str(&value) {
                                    parser.push(LayoutCommands::SetNumeric(local.clone(), value));
                                }
                            }
                            if let Some(value) = e.cdata("text") {
                                parser.push(LayoutCommands::SetText(local.clone(), value));
                            }
                            // if let Some(value) = e.cdata("image") {
                            //     if let Ok(value) = f32::from_str(&value) {
                            //         parser.push(LayoutCommands::Set(local.clone(), XMLType::Numeric(value)));
                            //     }
                            // }
                            if let Some(value) = e.cdata("color") {
                                let color = if let Ok(color) = csscolorparser::parse(&value) {
                                        let color = color.to_rgba8();
                                        let color = [color[0] as f32, color[1] as f32, color[2] as f32, color[3] as f32];
                                        color
                                } else { [0.0;4] };
                                parser.push(LayoutCommands::SetColor(local.clone(), color));
                            }
                        }
                    }
                    b"list-member" => {
                        if let Some(name) = e.cdata("name") {
                            parser.push(LayoutCommands::ListMember(name));
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

                        parser.push(LayoutCommands::Scroll(vertical, horizontal));
                    }
                    
                    other => return Err(ParserError::UnknownTag(other.to_owned()))
                }
            }
            Ok(Event::Text(e)) => parser.receive_text_content(e.unescape().unwrap().to_string()),
            _ => (),
        }
        buf.clear();
    }
    
    Ok(parser.flush())
}

pub fn set_layout<'render_pass, ImageElementData: Debug+Default, CustomElementData: Debug+Default, UserEvents: FromStr+Clone+PartialEq+Debug, UserApp: Get<ImageElementData, UserEvents>>(
    clicked: bool,
    events: &mut Vec<UserEvents>,
    xml_stack: &Vec<LayoutCommands<UserEvents>>,
    fragments: &HashMap<String, Vec<LayoutCommands<UserEvents>>>,
    layout_engine: &mut LayoutEngine<ImageElementData, CustomElementData, UserEvents>,
    user_app: &UserApp,
    locals: Option<&HashMap<String, XMLType<ImageElementData, UserEvents>>>
){
    let mut config = None::<ElementConfiguration>;
    let mut selected_fragment = None::<&Vec<LayoutCommands<UserEvents>>>;
    let mut local_call_stack = HashMap::<String, XMLType<ImageElementData, UserEvents>>::new();
    let mut text_config = None::<TextConfig>;
    let mut text_content = None::<&String>;
    let mut dynamic_text_content = None::<&'render_pass str>;
    let mut nesting_level = 0;
    let mut skip_level = -1;

    let mut list_stack = Vec::<LayoutCommands<UserEvents>>::new();
    let mut list_members = Vec::<LayoutCommands<UserEvents>>::new();
    let mut list_opened = false;
    let mut list_source = String::new();

    for xml_command in xml_stack.iter() {
        if list_opened {
            match xml_command {
                LayoutCommands::ListClosed => {
                    list_opened = false;
                }
                LayoutCommands::ListMember(_) => {
                    list_members.push(xml_command.clone())
                }
                other => {
                    list_stack.push(other.clone());
                    continue;
                }
            }
        }

        match xml_command {
            LayoutCommands::IfOpened(tag) => {
                if skip_level == -1 && !get_bool(&tag, user_app) {
                    skip_level = nesting_level;
                }

                nesting_level += 1;
            }
            LayoutCommands::IfClosed => {
                nesting_level -= 1;

                if skip_level == nesting_level {
                    skip_level = -1;
                }
            }
            LayoutCommands::HoveredOpened => {
                if skip_level == -1 && !layout_engine.hovered() {
                    skip_level = nesting_level;
                }

                nesting_level += 1;
            }
            LayoutCommands::HoveredClosed => {
                nesting_level -= 1;

                if skip_level == nesting_level {
                    skip_level = -1;
                }
            }
            LayoutCommands::ClickedOpened(event) => {
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
            LayoutCommands::ClickedClosed => {
                nesting_level -= 1;

                if skip_level == nesting_level {
                    skip_level = -1;
                }
            }
            LayoutCommands::ElementOpened(_id)=> {
                nesting_level += 1;

                if skip_level == -1 {
                    layout_engine.open();
                }
            }
            LayoutCommands::ElementClosed=> {
                nesting_level -= 1;

                if skip_level == -1 {
                    layout_engine.close();
                }
            }
            LayoutCommands::ConfigOpened if skip_level == -1 => {
                nesting_level += 1;

                if skip_level == -1 {
                    match config.is_some() {
                        false => config = Some(layout_engine.start_config()),
                        true => {} 
                    }
                }
            }
            LayoutCommands::ConfigClosed if skip_level == -1 => {
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
            
            LayoutCommands::CallOpened(fragment_name) if skip_level == -1 => {
                if let Some(fragment) = fragments.get(fragment_name) {
                    selected_fragment = Some(fragment);
                    local_call_stack.clear();
                }
            }
            LayoutCommands::CallClosed if skip_level == -1 => {
                if selected_fragment.is_some() {
                    if local_call_stack.len() > 0 {
                        set_layout(clicked, events, selected_fragment.take().unwrap(), fragments, layout_engine, user_app, Some(&local_call_stack));
                    }
                    else {
                        set_layout(clicked, events, selected_fragment.take().unwrap(), fragments, layout_engine, user_app, None);
                    }
                }
            }

            LayoutCommands::ListOpened(source) if skip_level == -1 => {
                list_source = source.to_string();
                list_stack.clear();
                list_members.clear();
                list_opened = true;
            }
            LayoutCommands::ListClosed if skip_level == -1 => {
                let list_length = user_app.get(&list_source);
                if let Some(source) = list_length {
                    if let XMLType::ListLength(length) = source {
                        for i in 0..length {
                            local_call_stack.clear();
                            for member in list_members.iter() {
                                if let LayoutCommands::ListMember(name) = member {
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
            
            LayoutCommands::Get(local, name) if skip_level == -1 => {
                if let Some(value) = user_app.get(name) {
                    local_call_stack.insert(local.to_string(), value);
                }
            }
            // todo other sets
            LayoutCommands::SetBool(local, value) if skip_level == -1 => {
                local_call_stack.insert(local.to_string(), XMLType::Bool(*value));
            }
            LayoutCommands::SetNumeric(local, value) if skip_level == -1 => {
                local_call_stack.insert(local.to_string(), XMLType::Numeric(*value));
            }
            LayoutCommands::SetText(local, value) if skip_level == -1 => {
                local_call_stack.insert(local.to_string(), XMLType::Text(&value.as_str()));
            }

            LayoutCommands::FitX if skip_level == -1 => {
                config.as_mut().unwrap().x_fit();
            }
            LayoutCommands::FitXmin(min) if skip_level == -1 => {
                config.as_mut().unwrap().x_fit_min(*min);
            }
            LayoutCommands::FitXminmax(min, max) if skip_level == -1 => {
                config.as_mut().unwrap().x_fit_min_max(*min, *max);
            }
            LayoutCommands::FitY if skip_level == -1 => {
                config.as_mut().unwrap().y_fit();
            }
            LayoutCommands::FitYmin(min) if skip_level == -1 => {
                config.as_mut().unwrap().y_fit_min(*min);
            }
            LayoutCommands::FitYminmax(min, max) if skip_level == -1 => {
                config.as_mut().unwrap().y_fit_min_max(*min, *max);
            }

            LayoutCommands::GrowX if skip_level == -1 => {
                config.as_mut().unwrap().x_grow();
            }
            LayoutCommands::GrowXmin(min) if skip_level == -1 => {
                config.as_mut().unwrap().x_grow_min(*min);
            }
            LayoutCommands::GrowXminmax(min, max) if skip_level == -1 => {
                config.as_mut().unwrap().x_grow_min_max(*min, *max);
            }
            LayoutCommands::GrowY if skip_level == -1 => {
                config.as_mut().unwrap().y_grow();
            }
            LayoutCommands::GrowYmin(min) if skip_level == -1 => {
                config.as_mut().unwrap().y_grow_min(*min);
            }
            LayoutCommands::GrowYminmax(min, max) if skip_level == -1 => {
                config.as_mut().unwrap().y_grow_min_max(*min, *max);
            }

            LayoutCommands::FixedX(x) if skip_level == -1 => {
                config.as_mut().unwrap().x_fixed(*x);
            }
            LayoutCommands::FixedY(y) if skip_level == -1 => {
                config.as_mut().unwrap().y_fixed(*y);
            }

            LayoutCommands::PercentX(size) if skip_level == -1 => {
                config.as_mut().unwrap().x_percent(*size);
            }
            LayoutCommands::PercentY(size) if skip_level == -1 => {
                config.as_mut().unwrap().y_percent(*size);
            }

            LayoutCommands::GrowAll if skip_level == -1 => {
                config.as_mut().unwrap().grow_all();
            }

            LayoutCommands::PaddingAll(padding) if skip_level == -1 => {
                config.as_mut().unwrap().padding_all(*padding);
            }
            LayoutCommands::PaddingTop(padding) if skip_level == -1 => {
                config.as_mut().unwrap().padding_top(*padding);
            }
            LayoutCommands::PaddingBottom(padding) if skip_level == -1 => {
                config.as_mut().unwrap().padding_bottom(*padding);
            }
            LayoutCommands::PaddingLeft(padding) if skip_level == -1 => {
                config.as_mut().unwrap().padding_left(*padding);
            }
            LayoutCommands::PaddingRight(padding) if skip_level == -1 => {
                config.as_mut().unwrap().padding_right(*padding);
            }

            LayoutCommands::DirectionTTB if skip_level == -1 => {
                config.as_mut().unwrap().direction(crate::layout::LayoutDirection::TopToBottom);
            }
            LayoutCommands::DirectionLTR if skip_level == -1 => {
                config.as_mut().unwrap().direction(crate::layout::LayoutDirection::LeftToRight);
            }

            LayoutCommands::Id(label) if skip_level == -1 => {
                config.as_mut().unwrap().id(&label);
            }

            LayoutCommands::ChildGap(gap) if skip_level == -1 => {
                config.as_mut().unwrap().child_gap(*gap);
            }

            LayoutCommands::ChildAlignmentXLeft if skip_level == -1 => {
                config.as_mut().unwrap().align_children_x(crate::layout::LayoutAlignmentX::Left);
            }
            LayoutCommands::ChildAlignmentXRight if skip_level == -1 => {
                config.as_mut().unwrap().align_children_x(crate::layout::LayoutAlignmentX::Right);
            }
            LayoutCommands::ChildAlignmentXCenter if skip_level == -1 => {
                config.as_mut().unwrap().align_children_x(crate::layout::LayoutAlignmentX::Center);
            }

            LayoutCommands::ChildAlignmentYTop if skip_level == -1 => {
                config.as_mut().unwrap().align_children_y(crate::layout::LayoutAlignmentY::Top);
            }
            LayoutCommands::ChildAlignmentYCenter if skip_level == -1 => {
                config.as_mut().unwrap().align_children_y(crate::layout::LayoutAlignmentY::Center);
            }
            LayoutCommands::ChildAlignmentYBottom if skip_level == -1 => {
                config.as_mut().unwrap().align_children_y(crate::layout::LayoutAlignmentY::Bottom);
            }

            LayoutCommands::Color(color) if skip_level == -1 => {
                config.as_mut().unwrap().background_color(*color);
            }

            LayoutCommands::RadiusAll(radius) if skip_level == -1 => {
                config.as_mut().unwrap().radius_all(*radius);
            }
            LayoutCommands::RadiusTopLeft(radius) if skip_level == -1 => {
                config.as_mut().unwrap().radius_top_left(*radius);
            }
            LayoutCommands::RadiusTopRight(radius) if skip_level == -1 => {
                config.as_mut().unwrap().radius_top_right(*radius);
            }
            LayoutCommands::RadiusBottomRight(radius) if skip_level == -1 => {
                config.as_mut().unwrap().radius_bottom_right(*radius);
            }
            LayoutCommands::RadiusBottomLeft(radius) if skip_level == -1 => {
                config.as_mut().unwrap().radius_bottom_left(*radius);
            }

            LayoutCommands::BorderAll(border, color) if skip_level == -1 => {
                config.as_mut().unwrap().border_all(*border, *color);
            }
            LayoutCommands::BorderTop(border, color) if skip_level == -1 => {
                config.as_mut().unwrap().border_top(*border, *color);
            }
            LayoutCommands::BorderBottom(border, color) if skip_level == -1 => {
                config.as_mut().unwrap().border_bottom(*border, *color);
            }
            LayoutCommands::BorderLeft(border, color) if skip_level == -1 => {
                config.as_mut().unwrap().border_left(*border, *color);
            }
            LayoutCommands::BorderRight(border, color) if skip_level == -1 => {
                config.as_mut().unwrap().border_right(*border, *color);
            }
            LayoutCommands::BorderBetweenChildren(border, color) if skip_level == -1 => {
                config.as_mut().unwrap().border_between_children(*border, *color);
            }

            LayoutCommands::TextOpened if skip_level == -1 => {
                nesting_level += 1;

                text_config = Some(layout_engine.open_text());
            } 
            LayoutCommands::TextClosed if skip_level == -1 => {
                match text_config.is_some() {
                    false => panic!("invalid xml stack"),
                    true => {
                        match text_content.is_some() {
                            false => {
                                match dynamic_text_content.is_some() {
                                    false => {
                                        let final_text_config = text_config.take().unwrap();
                                        layout_engine.close_text("", final_text_config.end());
                                    }
                                    true => {
                                        let final_text_config = text_config.take().unwrap();
                                        let final_dyn_content = dynamic_text_content.take().unwrap();
                                        layout_engine.close_text(&final_dyn_content, final_text_config.end());
                                    }
                                }
                            }
                            true => {
                                let final_text_config = text_config.take().unwrap();
                                layout_engine.close_text(text_content.take().unwrap(), final_text_config.end());
                            }
                        }
                        
                    },
                }

                nesting_level -= 1;
            }
            LayoutCommands::FontId(id) if skip_level == -1 => {
                text_config.as_mut().unwrap().font_id(*id);
            }
            LayoutCommands::TextAlignLeft if skip_level == -1 => {
                text_config.as_mut().unwrap().alignment(crate::text::TextAlignment::Left);
            }
            LayoutCommands::TextAlignRight if skip_level == -1 => {
                text_config.as_mut().unwrap().alignment(crate::text::TextAlignment::Right);
            }
            LayoutCommands::TextAlignCenter if skip_level == -1 => {
                text_config.as_mut().unwrap().alignment(crate::text::TextAlignment::Center);
            }
            LayoutCommands::TextLineHeight(lh) if skip_level == -1 => {
                text_config.as_mut().unwrap().line_height(*lh);
            }
            LayoutCommands::FontSize(size) if skip_level == -1 => {
                text_config.as_mut().unwrap().font_size(*size);
            }
            LayoutCommands::TextContent(content) if skip_level == -1 => {
                text_content = Some(content);
            }
            LayoutCommands::DynamicTextContent(content) if skip_level == -1 => {
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
            LayoutCommands::TextColor(color) if skip_level == -1 => {
                text_config.as_mut().unwrap().color(color[0], color[1], color[2], color[3]);
            }

            LayoutCommands::Scroll(vertical, horizontal) if skip_level == -1 => {
                config.as_mut().unwrap().scroll(*vertical, *horizontal);
            }

            _other => {}//println!("unused layout command: {:}", other);}
        }
    }
}