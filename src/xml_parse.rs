use std::{collections::HashMap, fmt::Debug, str::FromStr};

use csscolorparser;

use quick_xml::events::Event as XMLEvent;
use quick_xml::reader::Reader;
use quick_xml::events::BytesStart;
use quick_xml::Decoder;
use strum_macros::{Display, EnumString};

use crate::{Color, ElementConfiguration, LayoutEngine, TextConfig};

#[derive(Debug, Display)]
pub enum ParserError{
    RequiredAttributeValueMissing,
    UnNamedPage,
    UnSpecifiedIdTag,
    ListWithoutSource,
    UnNamedReusable,
    UnnamedUseTag,
    FileNotAccessable,
    ReaderError,
    UnknownTag(Vec<u8>),
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum LayoutCommandType<Event>
where
    Event: Clone+Debug+PartialEq
{
    FlowControl(FlowControlCommand),
    PageData(PageDataCommand<Event>),
    ElementConfig(ConfigCommand),
    TextConfig(TextConfigCommand),
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum FlowControlCommand{
    ElementOpened{id: Option<String>},
    ElementClosed,

    TextElementOpened,
    TextElementClosed,

    ConfigOpened,
    ConfigClosed,

    TextConfigOpened,
    TextConfigClosed,
    
    ListOpened{src: String},
    ListClosed,
    ListMember{name: String},

    UseOpened{name: String},
    UseClosed,

    IfOpened{condition: String},
    IfNotOpened{condition: String},
    IfClosed,

    HoveredOpened,
    HoveredClosed,

    // use clay_onhover and retreive the pointerdata from it
    ClickedOpened{event: Option<String>},
    ClickedClosed,
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum PageDataCommand<Event>
where
    Event: Clone+Debug+PartialEq
{
    SetBool{local: String, to:bool},
    SetNumeric{local: String, to:f32},
    SetText{local: String, to:String},
    SetColor{local: String, to:Color},
    SetEvent{local: String, to:Event},

    GetBool{local: String, from:String},
    GetNumeric{local: String, from:String},
    GetText{local: String, from:String},
    GetImage{local: String, from:String},
    GetColor{local: String, from:String},
    GetEvent{local: String, from:String},
}

impl<Event: Clone+Debug+PartialEq> PageDataCommand<Event>{
    fn get_local(&self) -> String {
        match self {
            Self::GetBool { local, from:_ } => local.to_string(),
            Self::GetNumeric { local, from:_ } => local.to_string(),
            Self::GetText { local, from:_ } => local.to_string(),
            Self::GetImage { local, from:_ } => local.to_string(),
            Self::GetColor { local, from:_ } => local.to_string(),
            Self::GetEvent { local, from:_ } => local.to_string(),
            
            Self::SetBool { local, to:_ } => local.to_string(),
            Self::SetNumeric { local, to:_ } => local.to_string(),
            Self::SetText { local, to:_ } => local.to_string(),
            Self::SetColor { local, to:_ } => local.to_string(),
            Self::SetEvent { local, to:_ } => local.to_string(),
        }
    }
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum ConfigCommand{
    Id(String),

    GrowAll,
    GrowX,
    GrowXmin{min: f32},
    GrowXminmax{min: f32, max:f32},
    GrowY,
    GrowYmin{min: f32},
    GrowYminmax{min: f32, max:f32},
    FitX,
    FitXmin{min: f32},
    FitXminmax{min: f32, max:f32},
    FitY,
    FitYmin{min: f32},
    FitYminmax{min: f32, max:f32},
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
    DynamicColor(String),

    RadiusAll(f32),
    RadiusTopLeft(f32),
    RadiusTopRight(f32),
    RadiusBottomRight(f32),
    RadiusBottomLeft(f32),

    BorderColor(Color),
    BorderDynamicColor(String),
    BorderAll(f32),
    BorderTop(f32),
    BorderLeft(f32),
    BorderBottom(f32),
    BorderRight(f32),
    BorderBetweenChildren(f32),

    Scroll(bool, bool),

    Image(String, f32, f32),

    // todo:
    // floating elements
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
    Reusable
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
enum AlignmentDirection {
    left,
    right,
    center,
    top,
    bottom,
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

#[allow(unused_variables)]
pub trait ParserDataAccess<Image, Event: FromStr+Clone+PartialEq>{
    fn get_bool(&self, name: &str) -> Option<bool>{
        None
    }
    fn get_numeric(&self, name: &str) -> Option<f32>{
        None
    }
    fn get_list_length(&self, name: &str) -> Option<i32>{
        None
    }
    fn get_text<'render_pass, 'application>(&'application self, name: &str) -> Option<&'render_pass str> where 'application: 'render_pass{
        None
    }
    fn get_image<'render_pass, 'application>(&'application self, name: &str ) -> Option<&'render_pass Image> where 'application: 'render_pass{
        None
    }
    fn get_color<'render_pass, 'application>(&'application self, name: &str ) -> Option<&'render_pass Color> where 'application: 'render_pass{
        None
    }
    fn get_event<'render_pass, 'application>(&'application self, name: &str ) -> Option<&'render_pass Event> where 'application: 'render_pass{
        None
    }
}

enum SsizeType {
    None,
    Min{min: f32},
    MinMax{min: f32, max:f32},
    At{at: f32}
}

fn parse<'a, T: FromStr+Default>(name: &str, bytes_start: &'a mut BytesStart) -> (T, bool) {
    if let Some(value) = bytes_start.cdata(name) {
        match T::from_str(&value) {
            Err(_) => (T::default(), true),
            Ok(value) => (value, true)
        }
    } else {(T::default(), false)}
}

fn try_parse<'a, T:FromStr>(name: &str, bytes_start: &'a mut BytesStart) -> Option<T> {
    if let Some(value) = bytes_start.cdata(name) {
        match T::from_str(&value) {
            Err(_) => None,
            Ok(value) => Some(value)
        }
    } else {None}
}

fn set_sizing_attributes<'a>(bytes_start: &'a mut BytesStart) -> SsizeType{
    let (min, min_exists) = parse::<f32>("min", bytes_start);
    let (max, max_exists) = parse::<f32>("max", bytes_start);
    let (at, at_exists) = parse::<f32>("at", bytes_start);

    if min_exists && max_exists {
        return SsizeType::MinMax { min, max }
    }
    else if min_exists {
        return SsizeType::Min { min }
    }
    else if at_exists {
        return SsizeType::At { at };
    }
    else {
        return SsizeType::None;
    }
}

#[derive(Default)]
pub struct Parser<Event>
where
    Event: Clone+Debug+PartialEq+FromStr
{
    mode: ParsingMode,

    current_page: Vec<LayoutCommandType<Event>>,
    current_page_name: String,
    pages: HashMap<String, Vec<LayoutCommandType<Event>>>,

    current_reusable: Vec<LayoutCommandType<Event>>,
    reusable_name: String,
    reusable: HashMap<String, Vec<LayoutCommandType<Event>>>,

    nesting_level: i32,
    xml_nesting_stack: Vec<i32>,

    text_opened: bool,
    text_content: Option<String>
}

impl<Event> Parser<Event>
where
    Event: Clone+Debug+PartialEq+FromStr
{
    pub fn add_page(&mut self, xml_string: &str) -> Result<(), ParserError>{
        let mut reader = Reader::from_str(xml_string);
        reader.config_mut().trim_text(true);
        let mut buf = Vec::<u8>::new();

        self.text_config(TextConfigCommand::DefaultText("hello".to_string()));

        loop {
            match reader.read_event_into(&mut buf) {
                Err(e) => {
                    println!("Reader Error: {:?}", e);
                    return Err(ParserError::ReaderError)
                },
                Ok(XMLEvent::Eof) => break,
                Ok(XMLEvent::Start(e)) => {
                    self.nest();
                    match e.name().as_ref() {
                        b"reusable" => match e.cdata("name") {
                            None => return Err(ParserError::UnNamedReusable),
                            Some(reusable_name) => self.open_reusable(reusable_name),
                        }
                        b"page" => match e.cdata("name") {
                            None => return Err(ParserError::UnNamedPage),
                            Some(name) => self.current_page_name = name
                        }
                        b"element" => {
                            if let Some(condition) = e.cdata("if") {
                                self.push_nest(FlowControlCommand::IfOpened{condition});
                            }
                            if let Some(condition) = e.cdata("if-not") {
                                self.push_nest(FlowControlCommand::IfNotOpened{condition});
                            }
                            self.flow_control(FlowControlCommand::ElementOpened{id:e.cdata("id")});
                        }
                        b"text-element" =>      self.flow_control(FlowControlCommand::TextElementOpened),
                        b"element-config" =>    self.flow_control(FlowControlCommand::ConfigOpened),
                        b"text-config" =>       self.flow_control(FlowControlCommand::TextConfigOpened),
                        b"content" =>           self.text_content = None,
                        b"use" => match e.cdata("name") {
                            None => return Err(ParserError::UnnamedUseTag),
                            Some(fragment_name) => self.flow_control(FlowControlCommand::UseOpened{name:fragment_name.clone()}),
                        }
                        b"hovered" =>           self.flow_control(FlowControlCommand::HoveredOpened),
                        b"clicked" =>           self.flow_control(FlowControlCommand::ClickedOpened{event: e.cdata("emit")}),
                        b"list" => match e.cdata("src") {
                            None => return Err(ParserError::ListWithoutSource),
                            Some(src) => self.flow_control(FlowControlCommand::ListOpened { src }),
                        }
                        other => return Err(ParserError::UnknownTag(other.to_owned()))
                    }
                }
                Ok(XMLEvent::End(e)) => {
                    self.denest();
                    match e.name().as_ref() {
                        b"reusable" =>          self.close_reusable(),
                        b"page" => (),
                        b"element" =>           self.try_pop_nest(),
                        b"element-config" =>    self.flow_control(FlowControlCommand::ConfigClosed),
                        b"text-element" =>      self.flow_control(FlowControlCommand::TextElementClosed),
                        b"content" =>           self.close_text_content(),
                        b"use" =>               self.flow_control(FlowControlCommand::UseClosed),
                        b"hovered" =>           self.flow_control(FlowControlCommand::HoveredClosed),
                        b"clicked" =>           self.flow_control(FlowControlCommand::ClickedClosed),
                        b"list" =>              self.flow_control(FlowControlCommand::ListClosed),
                        other => return Err(ParserError::UnknownTag(other.to_owned()))
                    }
                }
                Ok(XMLEvent::Empty(mut e)) => {
                    match e.name().as_ref() {
                        b"set-bool" => {
                            let local = match e.cdata("local") {
                                None => return Err(ParserError::RequiredAttributeValueMissing),
                                Some(value) => value
                            };
                            let to = match e.cdata("to") {
                                None => return Err(ParserError::RequiredAttributeValueMissing),
                                Some(value) => match bool::from_str(&value) {
                                    Err(_) => return Err(ParserError::RequiredAttributeValueMissing),
                                    Ok(value) => value
                                }
                            };
                            self.page_data(PageDataCommand::SetBool{local, to});
                        }
                        b"set-numeric" => {
                            let local = match e.cdata("local") {
                                None => return Err(ParserError::RequiredAttributeValueMissing),
                                Some(value) => value
                            };
                            let to = match e.cdata("to") {
                                None => return Err(ParserError::RequiredAttributeValueMissing),
                                Some(value) => match f32::from_str(&value) {
                                    Err(_) => return Err(ParserError::RequiredAttributeValueMissing),
                                    Ok(value) => value
                                }
                            };
                            self.page_data(PageDataCommand::SetNumeric{local, to});
                        }
                        b"set-text" => {
                            let local = match e.cdata("local") {
                                None => return Err(ParserError::RequiredAttributeValueMissing),
                                Some(value) => value
                            };
                            let to = match e.cdata("to") {
                                None => return Err(ParserError::RequiredAttributeValueMissing),
                                Some(value) => value
                            };
                            self.page_data(PageDataCommand::SetText{local, to});
                        }
                        b"set-color" => {
                            let local = match e.cdata("local") {
                                None => return Err(ParserError::RequiredAttributeValueMissing),
                                Some(value) => value
                            };
                            let to = match e.cdata("to") {
                                None => return Err(ParserError::RequiredAttributeValueMissing),
                                Some(value) => match csscolorparser::parse(&value) {
                                    Err(_) => return Err(ParserError::RequiredAttributeValueMissing),
                                    Ok(value) => value.to_rgba8().into()
                                }
                            };
                            self.page_data(PageDataCommand::SetColor{local, to});
                        }
                        b"set-event" => {
                            let local = match e.cdata("local") {
                                None => return Err(ParserError::RequiredAttributeValueMissing),
                                Some(value) => value
                            };
                            let to = match e.cdata("to") {
                                None => return Err(ParserError::RequiredAttributeValueMissing),
                                Some(value) => match Event::from_str(&value) {
                                    Err(_) => return Err(ParserError::RequiredAttributeValueMissing),
                                    Ok(value) => value
                                }
                            };
                            self.page_data(PageDataCommand::SetEvent{local, to});
                        }
                        b"get-bool" => {
                            let local = match e.cdata("local") {
                                None => return Err(ParserError::RequiredAttributeValueMissing),
                                Some(value) => value
                            };
                            let from = match e.cdata("from") {
                                None => return Err(ParserError::RequiredAttributeValueMissing),
                                Some(value) => value
                            };
                            self.page_data(PageDataCommand::GetBool { local, from });
                        }
                        b"get-numeric" => {
                            let local = match e.cdata("local") {
                                None => return Err(ParserError::RequiredAttributeValueMissing),
                                Some(value) => value
                            };
                            let from = match e.cdata("from") {
                                None => return Err(ParserError::RequiredAttributeValueMissing),
                                Some(value) => value
                            };
                            self.page_data(PageDataCommand::GetNumeric { local, from });
                        }
                        b"get-text" => {
                            let local = match e.cdata("local") {
                                None => return Err(ParserError::RequiredAttributeValueMissing),
                                Some(value) => value
                            };
                            let from = match e.cdata("from") {
                                None => return Err(ParserError::RequiredAttributeValueMissing),
                                Some(value) => value
                            };
                            self.page_data(PageDataCommand::GetText { local, from });
                        }
                        b"get-image" => {
                            let local = match e.cdata("local") {
                                None => return Err(ParserError::RequiredAttributeValueMissing),
                                Some(value) => value
                            };
                            let from = match e.cdata("from") {
                                None => return Err(ParserError::RequiredAttributeValueMissing),
                                Some(value) => value
                            };
                            self.page_data(PageDataCommand::GetImage { local, from });
                        }
                        b"get-color" => {
                            let local = match e.cdata("local") {
                                None => return Err(ParserError::RequiredAttributeValueMissing),
                                Some(value) => value
                            };
                            let from = match e.cdata("from") {
                                None => return Err(ParserError::RequiredAttributeValueMissing),
                                Some(value) => value
                            };
                            self.page_data(PageDataCommand::GetColor { local, from });
                        }
                        b"get-event" => {
                            let local = match e.cdata("local") {
                                None => return Err(ParserError::RequiredAttributeValueMissing),
                                Some(value) => value
                            };
                            let from = match e.cdata("from") {
                                None => return Err(ParserError::RequiredAttributeValueMissing),
                                Some(value) => value
                            };
                            self.page_data(PageDataCommand::GetEvent { local, from });
                        }
                        b"id" => match e.cdata("is") {
                            None => return Err(ParserError::UnSpecifiedIdTag),
                            Some(id) => self.config_command(ConfigCommand::Id(id)),
                        }
                        b"grow" => self.config_command(ConfigCommand::GrowAll),
                        b"width-fit" => match set_sizing_attributes(&mut e) {
                            SsizeType::MinMax { min, max } => self.config_command(ConfigCommand::FitXminmax { min, max }),
                            SsizeType::Min { min } => self.config_command(ConfigCommand::FitXmin { min }),
                            SsizeType::None => self.config_command(ConfigCommand::FitX),
                            _ => ()
                        }
                        b"width-grow" => match set_sizing_attributes(&mut e) {
                            SsizeType::MinMax { min, max } => self.config_command(ConfigCommand::GrowXminmax { min, max }),
                            SsizeType::Min { min } => self.config_command(ConfigCommand::GrowXmin { min }),
                            SsizeType::None => self.config_command(ConfigCommand::GrowX),
                            _ => ()
                        }
                        b"width-fixed" => if let SsizeType::At { at } = set_sizing_attributes(&mut e) {
                            self.config_command(ConfigCommand::FixedX(at));
                        }
                        b"width-percent" => if let SsizeType::At { at } = set_sizing_attributes(&mut e) {
                            self.config_command(ConfigCommand::PercentX(at));
                        }
                        b"height-fit" => match set_sizing_attributes(&mut e) {
                            SsizeType::MinMax { min, max } => self.config_command(ConfigCommand::FitYminmax { min, max }),
                            SsizeType::Min { min } => self.config_command(ConfigCommand::FitYmin { min }),
                            SsizeType::None => self.config_command(ConfigCommand::FitY),
                            _ => ()
                        }
                        b"height-grow" => match set_sizing_attributes(&mut e) {
                            SsizeType::MinMax { min, max } => self.config_command(ConfigCommand::GrowYminmax { min, max }),
                            SsizeType::Min { min } => self.config_command(ConfigCommand::GrowYmin { min }),
                            SsizeType::None => self.config_command(ConfigCommand::GrowY),
                            _ => ()
                        }
                        b"height-fixed" => if let SsizeType::At { at } = set_sizing_attributes(&mut e) {
                            self.config_command(ConfigCommand::FixedY(at));
                        }
                        b"height-percent" => if let SsizeType::At { at } = set_sizing_attributes(&mut e) {
                            self.config_command(ConfigCommand::PercentY(at));
                        }
                        b"padding-all" => match try_parse::<u16>("is", &mut e) {
                            None => return Err(ParserError::RequiredAttributeValueMissing),
                            Some(value) => self.config_command(ConfigCommand::PaddingAll(value)),
                        }
                        b"padding-left" => match try_parse::<u16>("is", &mut e) {
                            None => return Err(ParserError::RequiredAttributeValueMissing),
                            Some(value) => self.config_command(ConfigCommand::PaddingLeft(value)),
                        }
                        b"padding-bottom" => match try_parse::<u16>("is", &mut e) {
                            None => return Err(ParserError::RequiredAttributeValueMissing),
                            Some(value) => self.config_command(ConfigCommand::PaddingBottom(value)),
                        }
                        b"padding-right" => match try_parse::<u16>("is", &mut e) {
                            None => return Err(ParserError::RequiredAttributeValueMissing),
                            Some(value) => self.config_command(ConfigCommand::PaddingRight(value)),
                        }
                        b"padding-top" => match try_parse::<u16>("is", &mut e) {
                            None => return Err(ParserError::RequiredAttributeValueMissing),
                            Some(value) => self.config_command(ConfigCommand::PaddingTop(value)),
                        }
                        b"child-gap" => match try_parse::<u16>("is", &mut e) {
                            None => return Err(ParserError::RequiredAttributeValueMissing),
                            Some(value) => self.config_command(ConfigCommand::ChildGap(value)),
                        }
                        b"direction" => if let Some(direction) = e.cdata("is") {
                            if &direction == "ttb" {
                                self.config_command(ConfigCommand::DirectionTTB);
                            }
                            else {
                                self.config_command(ConfigCommand::DirectionLTR);
                            }
                        }
                        b"align-children-x" => if let Some(alignment) = e.cdata("to") {
                            match AlignmentDirection::from_str(&alignment) {
                                Err(_) => return Err(ParserError::RequiredAttributeValueMissing),
                                Ok(direction) => {
                                    match direction {
                                        AlignmentDirection::left => self.config_command(ConfigCommand::ChildAlignmentXLeft),
                                        AlignmentDirection::center => self.config_command(ConfigCommand::ChildAlignmentXCenter),
                                        AlignmentDirection::right => self.config_command(ConfigCommand::ChildAlignmentXRight),
                                        _ => {}
                                    }
                                }
                            }
                        }
                        b"align-children-y" => if let Some(alignment) = e.cdata("to") {
                            match AlignmentDirection::from_str(&alignment) {
                                Err(_) => return Err(ParserError::RequiredAttributeValueMissing),
                                Ok(direction) => {
                                    match direction {
                                        AlignmentDirection::top => self.config_command(ConfigCommand::ChildAlignmentYTop),
                                        AlignmentDirection::center => self.config_command(ConfigCommand::ChildAlignmentYCenter),
                                        AlignmentDirection::bottom => self.config_command(ConfigCommand::ChildAlignmentYBottom),
                                        _ => {}
                                    }
                                }
                            }
                        }
                        b"color" => if let Some(color) = e.cdata("is") {
                            match csscolorparser::parse(&color) {
                                Err(_) => self.config_command(ConfigCommand::Color(Color::default())),
                                Ok(color) => self.config_command(ConfigCommand::Color(color.to_rgba8().into())),
                            }
                        }
                        b"dyn-color" => if let Some(color) = e.cdata("from") {
                            self.config_command(ConfigCommand::DynamicColor(color));
                        }
                        b"radius-all" => match try_parse::<f32>("is", &mut e) {
                            None => return Err(ParserError::RequiredAttributeValueMissing),
                            Some(radius) => self.config_command(ConfigCommand::RadiusAll(radius)),
                        }
                        b"radius-top-left" => match try_parse::<f32>("is", &mut e) {
                            None => return Err(ParserError::RequiredAttributeValueMissing),
                            Some(radius) => self.config_command(ConfigCommand::RadiusTopLeft(radius)),
                        }
                        b"radius-top-right" => match try_parse::<f32>("is", &mut e) {
                            None => return Err(ParserError::RequiredAttributeValueMissing),
                            Some(radius) => self.config_command(ConfigCommand::RadiusTopLeft(radius)),
                        }
                        b"radius-bottom-left" => match try_parse::<f32>("is", &mut e) {
                            None => return Err(ParserError::RequiredAttributeValueMissing),
                            Some(radius) => self.config_command(ConfigCommand::RadiusBottomLeft(radius)),
                        }
                        b"radius-bottom-right" => match try_parse::<f32>("is", &mut e) {
                            None => return Err(ParserError::RequiredAttributeValueMissing),
                            Some(radius) => self.config_command(ConfigCommand::RadiusBottomRight(radius)),
                        }
                        b"border-color" => if let Some(color) = e.cdata("is") {
                            match csscolorparser::parse(&color) {
                                Err(_) => self.config_command(ConfigCommand::BorderColor(Color::default())),
                                Ok(color) => self.config_command(ConfigCommand::BorderColor(color.to_rgba8().into())),
                            }
                        }
                        b"border-dynamic-color" => match e.cdata("from") {
                            None => return Err(ParserError::RequiredAttributeValueMissing),
                            Some(color) => self.config_command(ConfigCommand::BorderDynamicColor(color)),
                        }
                        b"border-all" => match try_parse::<f32>("is", &mut e) {
                            None => return Err(ParserError::RequiredAttributeValueMissing),
                            Some(radius) => self.config_command(ConfigCommand::BorderAll(radius)),
                        }
                        b"border-top" => match try_parse::<f32>("is", &mut e) {
                            None => return Err(ParserError::RequiredAttributeValueMissing),
                            Some(radius) => self.config_command(ConfigCommand::BorderTop(radius)),
                        }
                        b"border-left" => match try_parse::<f32>("is", &mut e) {
                            None => return Err(ParserError::RequiredAttributeValueMissing),
                            Some(radius) => self.config_command(ConfigCommand::BorderLeft(radius)),
                        }
                        b"border-bottom" => match try_parse::<f32>("is", &mut e) {
                            None => return Err(ParserError::RequiredAttributeValueMissing),
                            Some(radius) => self.config_command(ConfigCommand::BorderBottom(radius)),
                        }
                        b"border-right" => match try_parse::<f32>("is", &mut e) {
                            None => return Err(ParserError::RequiredAttributeValueMissing),
                            Some(radius) => self.config_command(ConfigCommand::BorderRight(radius)),
                        }
                        b"border-between-children" => match try_parse::<f32>("is", &mut e) {
                            None => return Err(ParserError::RequiredAttributeValueMissing),
                            Some(radius) => self.config_command(ConfigCommand::BorderBetweenChildren(radius)),
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
    
                            self.config_command(ConfigCommand::Scroll(vertical, horizontal));
                        }
                        // todo:
                        // - image
                        // - custom element
                        // - floating
                        // - custom layout
                        b"font-id" => match try_parse::<u16>("is", &mut e) {
                            None => return Err(ParserError::RequiredAttributeValueMissing),
                            Some(id) => self.text_config(TextConfigCommand::FontId(id)),
                        }
                        b"text-align-left" => self.text_config(TextConfigCommand::AlignLeft),
                        b"text-align-right" => self.text_config(TextConfigCommand::AlignRight),
                        b"text-align-center" => self.text_config(TextConfigCommand::AlignCenter),
                        b"font-size" => match try_parse::<u16>("is", &mut e) {
                            None => return Err(ParserError::RequiredAttributeValueMissing),
                            Some(value) => self.text_config(TextConfigCommand::FontSize(value)),
                        }
                        b"line-height" => match try_parse::<u16>("is", &mut e) {
                            None => return Err(ParserError::RequiredAttributeValueMissing),
                            Some(value) => self.text_config(TextConfigCommand::LineHeight(value)),
                        }
                        b"dyn-content" => match e.cdata("from") {
                            None => return Err(ParserError::RequiredAttributeValueMissing),
                            Some(name) => self.text_config(TextConfigCommand::DynamicContent(name)),
                        }
                        other => return Err(ParserError::UnknownTag(other.to_owned()))
                    }
                }
                Ok(XMLEvent::Text(e)) => self.receive_text_content(e.unescape().unwrap().to_string()),
                _ => ()
            }
        }
        Ok(())
    }
    fn flow_control(&mut self, command: FlowControlCommand){
        match command {
            FlowControlCommand::TextConfigOpened => self.text_opened = true,
            FlowControlCommand::TextConfigClosed => self.text_opened = false,
            _ => ()
        }
        match self.mode {
            ParsingMode::Reusable => self.current_reusable.push(LayoutCommandType::FlowControl(command)),
            ParsingMode::Normal => self.current_page.push(LayoutCommandType::FlowControl(command)),
        }
    }
    fn page_data(&mut self, command: PageDataCommand<Event>){
        match self.mode {
            ParsingMode::Reusable => self.current_reusable.push(LayoutCommandType::PageData(command)),
            ParsingMode::Normal => self.current_page.push(LayoutCommandType::PageData(command)),
        }
    }
    fn config_command(&mut self, command: ConfigCommand){
        match self.mode {
            ParsingMode::Reusable => self.current_reusable.push(LayoutCommandType::ElementConfig(command)),
            ParsingMode::Normal => self.current_page.push(LayoutCommandType::ElementConfig(command)),
        }
    }
    fn text_config(&mut self, command: TextConfigCommand){
        match self.mode {
            ParsingMode::Reusable => self.current_reusable.push(LayoutCommandType::TextConfig(command)),
            ParsingMode::Normal => self.current_page.push(LayoutCommandType::TextConfig(command)),
        }
    }
    fn push_nest(&mut self, tag: FlowControlCommand){
        self.flow_control(tag);
        self.xml_nesting_stack.push(self.nesting_level);
    }
    fn try_pop_nest(&mut self){
        self.flow_control(FlowControlCommand::ElementClosed);
        match self.xml_nesting_stack.last() {
            None => {}
            Some(saved_nesting_level) => {
                if self.nesting_level < *saved_nesting_level {
                    self.xml_nesting_stack.pop();
                    self.flow_control(FlowControlCommand::IfClosed);
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
    fn receive_text_content(&mut self, content: String){
        if self.text_opened {
            self.text_content = Some(content);
        }
    }
    fn close_text_content(&mut self){
        let content = self.text_content.take().unwrap();
        self.text_config(TextConfigCommand::Content(content));
    }
    fn open_reusable(&mut self, name: String){
        self.reusable_name = name;
        self.current_reusable.clear();
        self.mode = ParsingMode::Reusable;
    }
    fn close_reusable(&mut self){
        let new_fragment = self.current_reusable.clone();
        self.reusable.insert(self.reusable_name.clone(), new_fragment);
        self.mode = ParsingMode::Normal;
    }

    // pub fn set_page<'render_pass, ImageElementData: Debug+Default, CustomElementData: Debug+Default, CustomEvent: FromStr+Clone+PartialEq, UserApp: ParserDataAccess<ImageElementData, CustomEvent>>(
    //     page: &str,
    //     clicked: bool,
    //     events: &mut Vec<CustomEvent>,
    //     layout_engine: &mut LayoutEngine<ImageElementData, CustomElementData, CustomEvent>,
    //     user_app: &UserApp
    // ){}
}

fn set_layout<'render_pass, Image, Event, Custom, UserApp: ParserDataAccess<Image, Event>>(
    events: &mut Vec<Event>,
    commands: &Vec<LayoutCommandType<Event>>,
    reusables: &HashMap<String, Vec<LayoutCommandType<Event>>>,
    locals: Option<&HashMap<String, &PageDataCommand<Event>>>,
    layout_engine: &mut LayoutEngine<Image, Custom, Event>,
    user_app: &UserApp,
)
where 
    Image: Clone+Debug+Default+PartialEq, 
    Event: FromStr+Clone+PartialEq+Debug, 
    Custom: Debug+Default
{
    let mut local_call_stack = HashMap::<String, &PageDataCommand<Event>>::new();
    let mut nesting_level = 0;
    let mut skip_level = -1;

    let mut selected_fragment = None::<&Vec<LayoutCommandType<Event>>>;

    let mut list_commands = Vec::<LayoutCommandType<Event>>::new();
    let mut list_locals = Vec::<PageDataCommand<Event>>::new();
    let mut list_opened = false;
    let mut list_source = String::new();
    let mut declare = false;
    
    let mut config = None::<ElementConfiguration>;
    
    let mut text_config = None::<TextConfig>;
    let mut text_content = None::<&String>;
    let mut dynamic_text_content = None::<&'render_pass str>;

    for command in commands.iter() {
        if list_opened {
            match command {
                LayoutCommandType::FlowControl(flow_command) if *flow_command == FlowControlCommand::ListClosed => list_opened = false,
                LayoutCommandType::PageData(data_command) if declare => {
                    list_locals.push(data_command.clone());
                    continue;
                }
                other => {
                    declare = false;
                    list_commands.push(other.clone());
                    continue;
                }
            }
        }

        match command {
            LayoutCommandType::FlowControl(control_command) => {
                match control_command {
                    FlowControlCommand::IfOpened { condition } => {
                        if skip_level == -1 {
                            if let Some(value) = user_app.get_bool(&condition) {
                                if !value {
                                    skip_level = nesting_level
                                }
                            }
                        }
        
                        nesting_level += 1;
                    }
                    FlowControlCommand::IfClosed => {
                        nesting_level -= 1;

                        if skip_level == nesting_level {
                            skip_level = -1;
                        }
                    }
                    FlowControlCommand::HoveredOpened => {
                        if skip_level == -1 && !layout_engine.hovered() {
                            skip_level = nesting_level;
                        }
        
                        nesting_level += 1;
                    }
                    FlowControlCommand::HoveredClosed => {
                        nesting_level -= 1;

                        if skip_level == nesting_level {
                            skip_level = -1;
                        }
                    }
                    FlowControlCommand::ClickedOpened { event } => {

                        

                        if skip_level == -1 && (!clicked || !layout_engine.hovered()) {
                            skip_level = nesting_level;
                        }
                        // TODO: check for local event substitution:
                        //
                        // else {
                        //     if let Some(event) = event {
                        //         events.push(event.clone());
                        //     }
                        // }
        
                        nesting_level += 1;
                    }
                    FlowControlCommand::ListOpened { src } => {
                        list_source = src.to_string();
                        list_commands.clear();
                        list_locals.clear();
                        list_opened = true;
                        declare = true;
                    }
                    FlowControlCommand::ListClosed => {
                        let list_length = user_app.get_list_length(&list_source);
                        if let Some(source) = list_length {
                            for i in 0..source {
                                local_call_stack.clear();
                                for member in list_locals.iter() {
                                    let name = member.get_local();
                                    local_call_stack.insert(name, member);
                                }
                                set_layout(events,  &list_commands, reusables,Some(&local_call_stack), layout_engine, user_app );
                            }
                        }
                    }
                    _ => {}
                }
            }
            LayoutCommandType::PageData(data_command) => {}
            LayoutCommandType::ElementConfig(config_command) => {}
            LayoutCommandType::TextConfig(config_command) => {}
        }
    }

    for xml_command in commands.iter() {
        match xml_command {
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
                if let Some(fragment) = reusables.get(fragment_name) {
                    selected_fragment = Some(fragment);
                    local_call_stack.clear();
                }
            }
            ConfigCommand::CallClosed  => {
                if selected_fragment.is_some() {
                    if local_call_stack.len() > 0 {
                        set_layout(clicked, events, selected_fragment.take().unwrap(), reusables, layout_engine, user_app, Some(&local_call_stack));
                    }
                    else {
                        set_layout(clicked, events, selected_fragment.take().unwrap(), reusables, layout_engine, user_app, None);
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