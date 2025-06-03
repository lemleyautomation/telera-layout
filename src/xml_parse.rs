use std::hash::Hash;
use std::{collections::HashMap, fmt::Debug, str::FromStr};

use csscolorparser;

use quick_xml::events::Event as XMLEvent;
use quick_xml::reader::Reader;
use quick_xml::events::BytesStart;
use quick_xml::Decoder;

pub use strum;
pub use strum_macros::Display;
pub use strum_macros::EnumString;

use crate::{Color, ElementConfiguration, LayoutEngine, MeasureText, TextConfig};

#[derive(Debug, Display)]
pub enum ParserError{
    RequiredAttributeValueMissing,
    DynamicAndStaticValues,
    UnNamedPage,
    UnSpecifiedIdTag,
    ListWithoutSource,
    UnNamedReusable,
    UnnamedUseTag,
    FileNotAccessable,
    ReaderError,
    UnknownTag(String),
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

    UseOpened{name: String},
    UseClosed,

    // if not
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
    DynamicId(String),
    StaticId(String),

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

    Clip{vertical: bool, horizontal: bool},

    Image{name: String},

    Floating,
    FloatingOffset{x:f32,y:f32},
    FloatingDimensions{width:f32,height:f32},
    FloatingZIndex{z:i16},
    FloatingAttatchToParentAtTopLeft,
    FloatingAttatchToParentAtCenterLeft,
    FloatingAttatchToParentAtBottomLeft,
    FloatingAttatchToParentAtTopCenter,
    FloatingAttatchToParentAtCenter,
    FloatingAttatchToParentAtBottomCenter,
    FloatingAttatchToParentAtTopRight,
    FloatingAttatchToParentAtCenterRight,
    FloatingAttatchToParentAtBottomRight,
    FloatingAttatchElementAtTopLeft,
    FloatingAttatchElementAtCenterLeft,
    FloatingAttatchElementAtBottomLeft,
    FloatingAttatchElementAtTopCenter,
    FloatingAttatchElementAtCenter,
    FloatingAttatchElementAtBottomCenter,
    FloatingAttatchElementAtTopRight,
    FloatingAttatchElementAtCenterRight,
    FloatingAttatchElementAtBottomRight,
    FloatingPointerPassThrough,
    FloatingAttachElementToElement{other_element_id:String},
    FloatingAttachElementToRoot,

    // todo:
    // floating elements
    // custom elements
    // custom layouts
}

pub struct ListData<'list_iteration>{
    pub src: &'list_iteration str,
    pub index: i32,
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
    Editable(bool),
    Content(String),
    DynamicContent(String),
    Color(Color),
}

#[derive(Default, Debug)]
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
    fn get_bool(&self, name: &str, list: &Option<ListData>) -> Option<bool>{
        None
    }
    fn get_numeric(&self, name: &str, list: &Option<ListData>) -> Option<f32>{
        None
    }
    fn get_list_length(&self, name: &str, list: &Option<ListData>) -> Option<i32>{
        None
    }
    fn get_text<'render_pass, 'application>(&'application self, name: &str, list: &Option<ListData>) -> Option<&'render_pass str> where 'application: 'render_pass{
        None
    }
    fn get_image<'render_pass, 'application>(&'application self, name: &str, list: &Option<ListData> ) -> Option<&'render_pass Image> where 'application: 'render_pass{
        None
    }
    fn get_color<'render_pass, 'application>(&'application self, name: &str, list: &Option<ListData> ) -> Option<&'render_pass Color> where 'application: 'render_pass{
        None
    }
    fn get_event<'render_pass, 'application>(&'application self, name: &str, list: &Option<ListData> ) -> Option<Event> where 'application: 'render_pass{
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

enum ValueRef{
    Dynamic(String),
    Static(String)
}

fn dyn_or_stat<'a>(e: &'a mut BytesStart) -> Result<ValueRef, ParserError>{
    let static_value = e.cdata("is");
    let dynamic_value = e.cdata("from");

    match ((static_value.is_some() as u8)*2) + (dynamic_value.is_some() as u8) {
        0 => return Err(ParserError::RequiredAttributeValueMissing),
        1 => return Ok(ValueRef::Dynamic(dynamic_value.unwrap())),
        2 => return Ok(ValueRef::Static(static_value.unwrap())),
        3 => return Err(ParserError::DynamicAndStaticValues),
        _ => return Err(ParserError::RequiredAttributeValueMissing),
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct XMLPage<Event>
where
    Event: Clone+Debug+PartialEq+FromStr,
{
    commands: Vec<LayoutCommandType<Event>>,
    reusables: HashMap<String, Vec<LayoutCommandType<Event>>>,
    editable_text: HashMap<String, u32>
}

#[derive(Default, Debug)]
pub struct Parser<Event, Page>
where
    Event: Clone+Debug+PartialEq+FromStr,
    Page: FromStr+Clone+Hash+Eq,
{
    pages: HashMap<Page, Vec<LayoutCommandType<Event>>>,
    reusable: HashMap<String, Vec<LayoutCommandType<Event>>>,

    page: HashMap<Page, XMLPage<Event>>,

    mode: ParsingMode,

    current_page: Vec<LayoutCommandType<Event>>,
    current_page_name: Page,

    current_reusable: Vec<LayoutCommandType<Event>>,
    reusable_name: String,

    nesting_level: i32,
    xml_nesting_stack: Vec<i32>,

    text_opened: bool,
    text_content: Option<String>,
}

impl<Event, Page> Parser<Event, Page>
where
    Event: Clone+Debug+PartialEq+FromStr,
    <Event as FromStr>::Err: Debug,
    Page: FromStr+Clone+Hash+Eq,
    <Page as FromStr>::Err: Debug
{
    pub fn update_page(&mut self, xml_string: &str){
        let pages_copy = self.pages.clone();
        let reusables_compy = self.reusable.clone();
        self.pages.clear();
        self.reusable.clear();

        match self.add_page(xml_string) {
            Ok(()) => {},
            Err(_) => {
                println!("! <------------------invalid layout file------------------/>");
                self.pages.clear();
                self.reusable.clear();
                self.pages = pages_copy;
                self.reusable = reusables_compy;
            }
        }
    }

    pub fn add_page(&mut self, xml_string: &str) -> Result<(), ParserError>{
        let mut reader = Reader::from_str(xml_string);
        reader.config_mut().trim_text(true);
        let mut buf = Vec::<u8>::new();

        let mut text_opened = false;

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
                            Some(name) => self.current_page_name = Page::from_str(&name).unwrap()
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
                        b"text-element" =>      {
                            self.flow_control(FlowControlCommand::TextElementOpened);
                            text_opened = true;
                        }
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
                        other => return Err(ParserError::UnknownTag(unsafe{String::from_raw_parts(other.to_owned().as_mut_ptr(), other.len(), other.len()*2)}))
                    }
                }
                Ok(XMLEvent::End(e)) => {
                    self.denest();
                    match e.name().as_ref() {
                        b"reusable" =>          self.close_reusable(),
                        b"page" => (),
                        b"element" =>           self.try_pop_nest(),
                        b"element-config" =>    self.flow_control(FlowControlCommand::ConfigClosed),
                        b"text-config" =>       self.flow_control(FlowControlCommand::TextConfigClosed),
                        b"text-element" =>      {
                            self.flow_control(FlowControlCommand::TextElementClosed);
                            text_opened = false;
                        }
                        b"content" =>           self.close_text_content(),
                        b"use" =>               self.flow_control(FlowControlCommand::UseClosed),
                        b"hovered" =>           self.flow_control(FlowControlCommand::HoveredClosed),
                        b"clicked" =>           self.flow_control(FlowControlCommand::ClickedClosed),
                        b"list" =>              self.flow_control(FlowControlCommand::ListClosed),
                        other => return Err(ParserError::UnknownTag(unsafe{String::from_raw_parts(other.to_owned().as_mut_ptr(), other.len(), other.len()*2)}))
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
                        b"id" => {
                            match dyn_or_stat(&mut e).unwrap() {
                                ValueRef::Dynamic(dyn_id) => self.config_command(ConfigCommand::DynamicId(dyn_id)),
                                ValueRef::Static(stat_id) => self.config_command(ConfigCommand::StaticId(stat_id)),
                            }
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
                            if text_opened {
                                match csscolorparser::parse(&color) {
                                    Err(_) => self.text_config(TextConfigCommand::Color(Color::default())),
                                    Ok(color) => self.text_config(TextConfigCommand::Color(color.to_rgba8().into())),
                                }
                            }
                            else {
                                match csscolorparser::parse(&color) {
                                    Err(_) => self.config_command(ConfigCommand::Color(Color::default())),
                                    Ok(color) => self.config_command(ConfigCommand::Color(color.to_rgba8().into())),
                                }
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
    
                            self.config_command(ConfigCommand::Clip{vertical, horizontal});
                        }
                        b"image" => {
                            let name = e.cdata("src");

                            if name.is_some() {
                                let name = name.unwrap();

                                self.config_command(ConfigCommand::Image { name });
                            }
                        }
                        // todo:
                        // - custom element
                        // - custom layout
                        b"floating" => {
                            self.config_command(ConfigCommand::Floating);
                        }
                        b"floating-offset" => {
                            let (x, x_exists) = parse::<f32>("x", &mut e);
                            let (y, y_exists) = parse::<f32>("y", &mut e);
                            if x_exists && y_exists {
                                self.config_command(ConfigCommand::FloatingOffset { x, y });
                            }
                        }
                        b"floating-size" => {
                            let (width, width_exists) = parse::<f32>("width", &mut e);
                            let (height, height_exists) = parse::<f32>("height", &mut e);
                            if width_exists && height_exists {
                                self.config_command(ConfigCommand::FloatingDimensions { width, height });
                            }
                        }
                        b"floating-z-index" => {
                            let (z, z_exists) = parse::<i16>("z", &mut e);
                            if z_exists {
                                self.config_command(ConfigCommand::FloatingZIndex { z });
                            }
                        }
                        b"floating-attach-to-parent" => {
                            if e.cdata("top-left").is_some() {
                                self.config_command(ConfigCommand::FloatingAttatchToParentAtTopLeft);
                                continue
                            }
                            if e.cdata("center-left").is_some() {
                                self.config_command(ConfigCommand::FloatingAttatchToParentAtCenterLeft);
                                continue
                            }
                            if e.cdata("bottom-left").is_some() {
                                self.config_command(ConfigCommand::FloatingAttatchToParentAtBottomLeft);
                                continue
                            }
                            if e.cdata("top-center").is_some() {
                                self.config_command(ConfigCommand::FloatingAttatchToParentAtTopCenter);
                                continue
                            }
                            if e.cdata("center").is_some() {
                                self.config_command(ConfigCommand::FloatingAttatchToParentAtCenter);
                                continue
                            }
                            if e.cdata("bottom-center").is_some() {
                                self.config_command(ConfigCommand::FloatingAttatchToParentAtBottomCenter);
                                continue
                            }
                            if e.cdata("top-right").is_some() {
                                self.config_command(ConfigCommand::FloatingAttatchToParentAtTopRight);
                                continue
                            }
                            if e.cdata("center-right").is_some() {
                                self.config_command(ConfigCommand::FloatingAttatchToParentAtCenterRight);
                                continue
                            }
                            if e.cdata("bottom-right").is_some() {
                                self.config_command(ConfigCommand::FloatingAttatchToParentAtBottomRight);
                                continue
                            }
                        }
                        b"floating-attach-element" => {
                            if e.cdata("top-left").is_some() {
                                self.config_command(ConfigCommand::FloatingAttatchElementAtTopLeft);
                                continue
                            }
                            if e.cdata("center-left").is_some() {
                                self.config_command(ConfigCommand::FloatingAttatchElementAtCenterLeft);
                                continue
                            }
                            if e.cdata("bottom-left").is_some() {
                                self.config_command(ConfigCommand::FloatingAttatchElementAtBottomLeft);
                                continue
                            }
                            if e.cdata("top-center").is_some() {
                                self.config_command(ConfigCommand::FloatingAttatchElementAtTopCenter);
                                continue
                            }
                            if e.cdata("center").is_some() {
                                self.config_command(ConfigCommand::FloatingAttatchElementAtCenter);
                                continue
                            }
                            if e.cdata("bottom-center").is_some() {
                                self.config_command(ConfigCommand::FloatingAttatchElementAtBottomCenter);
                                continue
                            }
                            if e.cdata("top-right").is_some() {
                                self.config_command(ConfigCommand::FloatingAttatchElementAtTopRight);
                                continue
                            }
                            if e.cdata("center-right").is_some() {
                                self.config_command(ConfigCommand::FloatingAttatchElementAtCenterRight);
                                continue
                            }
                            if e.cdata("bottom-right").is_some() {
                                self.config_command(ConfigCommand::FloatingAttatchElementAtBottomRight);
                                continue
                            }
                        }
                        b"floating-capture-pointer" => {
                            let (state, state_exists) = parse::<bool>("state", &mut e);
                            if state_exists && !state {
                                self.config_command(ConfigCommand::FloatingPointerPassThrough);
                            }
                        }
                        b"floating-attach-to-element" => {
                            let (other_element_id, exists) = parse::<String>("id", &mut e);
                            if exists {
                                self.config_command(ConfigCommand::FloatingAttachElementToElement { other_element_id });
                            }
                        }
                        b"floating-attach-to-root" => {
                            self.config_command(ConfigCommand::FloatingAttachElementToRoot);
                        }
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
                        b"editable" => self.text_config(TextConfigCommand::Editable(true)),
                        b"dyn-content" => match e.cdata("from") {
                            None => return Err(ParserError::RequiredAttributeValueMissing),
                            Some(name) => self.text_config(TextConfigCommand::DynamicContent(name)),
                        }
                        other => return Err(ParserError::UnknownTag(unsafe{String::from_raw_parts(other.to_owned().as_mut_ptr(), other.len(), other.len()*2)}))
                    }
                }
                Ok(XMLEvent::Text(e)) => {
                    self.receive_text_content(e.unescape().unwrap().to_string())
                },
                _ => ()
            }
        }
        
        match self.page.contains_key(&self.current_page_name) {
            true => self.page.remove(&self.current_page_name),
            false => self.page.insert(self.current_page_name.clone(),
            XMLPage {
                    commands: self.current_page.clone(),
                    reusables: self.reusable.clone(),                     
                    editable_text: HashMap::new(),
                }
            )
        };
        
        self.pages.insert(self.current_page_name.clone(), self.current_page.clone());
        self.current_page.clear();
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
        self.text_content = Some(content);
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

    pub fn set_page<'render_pass, Renderer, Image, Custom, CustomLayout, UserApp>(
        &mut self,
        page: &Page,
        clicked: bool,
        layout_engine: &mut LayoutEngine<Renderer, Image, Custom, CustomLayout>,
        user_app: &UserApp
    ) -> Vec<Event>
    where 
        Renderer: MeasureText,
        Image: Clone+Debug+Default+PartialEq, 
        Event: FromStr+Clone+PartialEq+Debug+Default, 
        Custom: Debug+Default,
        UserApp: ParserDataAccess<Image, Event>
    {
        let mut events = Vec::<Event>::new();
        if let Some(page_commands) = self.pages.get(page) {
            let mut command_references = Vec::<&LayoutCommandType<Event>>::new();
            for command in page_commands.iter() {
                command_references.push(command);
            }
            set_layout(clicked, &mut events, &command_references, &self.reusable, None, None, &mut None, &mut None, layout_engine, user_app);
        }
        events
    }
}

fn set_layout<'render_pass, Renderer: MeasureText, Image, Event, Custom, CustomLayout, UserApp>(
    clicked: bool,
    events: &mut Vec<Event>,
    commands: &Vec<&LayoutCommandType<Event>>,
    reusables: &HashMap<String, Vec<LayoutCommandType<Event>>>,
    locals: Option<&HashMap<String, &PageDataCommand<Event>>>,
    list_data: Option<ListData>,
    append_config: &mut Option<ElementConfiguration>,
    append_text_config: &mut Option<TextConfig>,
    layout_engine: &mut LayoutEngine<Renderer, Image, Custom, CustomLayout>,
    user_app: &UserApp,
)
where 
    Image: Clone+Debug+Default+PartialEq, 
    Event: FromStr+Clone+PartialEq+Debug+Default,
    <Event as FromStr>::Err: Debug,
    Custom: Debug+Default,
    UserApp: ParserDataAccess<Image, Event>
{
    #[cfg(feature="parse_logger")]
    if let Some(list_data) = &list_data {
        println!("list src:{:?}, list index:{:?}", &list_data.src, &list_data.index);
        if let Some(locals) = locals {
            for key in locals.keys() {
                println!("{:}", key);
            }
        }
    }

    let mut nesting_level: u32 = 0;
    let mut skip: Option<u32> = None;

    let mut recursive_commands = Vec::<&LayoutCommandType<Event>>::new();
    let mut recursive_source = String::new();
    let mut recursive_call_stack = HashMap::<String, &PageDataCommand<Event>>::new();
    let mut collect_recursive_declarations = false;

    let mut collect_list_commands = false;
    
    // let mut config = match append_config.is_some() {
    //     false => None::<ElementConfiguration>,
    //     true => *append_config
    // };
    let mut config = None::<ElementConfiguration>;
    
    let mut text_config = match append_text_config.is_some() {
        false => None::<TextConfig>,
        true => *append_text_config
    };

    let mut text_content = None::<&String>;
    let mut dynamic_text_content = None::<&'render_pass str>;

    for command in commands.iter() {
        #[cfg(feature="parse_logger")]
        println!("skip active: {:?}, {:?}", &skip, command);
        if collect_list_commands {
            match command {
                LayoutCommandType::FlowControl(flow_command) if *flow_command == FlowControlCommand::ListClosed => collect_list_commands = false,
                LayoutCommandType::PageData(_) => {}
                other => {
                    collect_recursive_declarations = false;
                    recursive_commands.push(other);
                    continue;
                }
            }
        }

        match command {
            LayoutCommandType::FlowControl(control_command) => {
                match control_command {
                    FlowControlCommand::IfOpened { condition } => {
                        if skip.is_none() {
                            match locals {
                                None => if let Some(value) = user_app.get_bool(&condition, &None) {
                                    if !value {
                                        skip = Some(nesting_level)
                                    }
                                }
                                Some(locals) =>  match locals.get(condition) {
                                    None => if let Some(value) = user_app.get_bool(&condition, &None) {
                                        if !value {
                                            skip = Some(nesting_level)
                                        }
                                    }
                                    Some(data_command) => {
                                        match data_command {
                                            PageDataCommand::GetBool { local:_, from } =>  if let Some(value) = user_app.get_bool (from, &list_data) {
                                                if !value {
                                                    skip = Some(nesting_level)
                                                }
                                            }
                                            PageDataCommand::SetBool { local:_, to } => if !to {
                                                skip = Some(nesting_level)
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                            }
                        }
                        nesting_level += 1;
                    }
                    FlowControlCommand::IfNotOpened { condition } => {
                        if skip.is_none() {
                            match locals {
                                None => if let Some(value) = user_app.get_bool(&condition, &None) {
                                    if value {
                                        skip = Some(nesting_level)
                                    }
                                }
                                Some(locals) =>  {
                                    match locals.get(condition) {
                                        None => if let Some(value) = user_app.get_bool(&condition, &None) {
                                            if value {
                                                skip = Some(nesting_level)
                                            }
                                        }
                                        Some(data_command) => {
                                            match data_command {
                                                PageDataCommand::GetBool { local:_, from } =>  {
                                                    if let Some(value) = user_app.get_bool (from, &list_data) {
                                                        #[cfg(feature="parse_logger")]
                                                        println!("if not {:?} @index({:?}) = {:?}", from, &list_data.as_ref().unwrap().index, value);
                                                        if value {
                                                            skip = Some(nesting_level)
                                                        }
                                                    }
                                                    else {
                                                        
                                                    }
                                                }
                                                PageDataCommand::SetBool { local:_, to } => if *to {
                                                    skip = Some(nesting_level)
                                                }
                                                _ => {}
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        nesting_level += 1;
                    }
                    FlowControlCommand::IfClosed => {
                        nesting_level -= 1;

                        if let Some(skip_level) = skip {
                            #[cfg(feature="parse_logger")]
                            println!("trying to close skip: {:?}, {:?}", skip_level, nesting_level);
                            if skip_level <= nesting_level{
                                skip = None;
                            }
                        }
                    }
                    FlowControlCommand::HoveredOpened => {
                        if skip.is_none() && !layout_engine.hovered() {
                            skip = Some(nesting_level);
                        }
        
                        nesting_level += 1;
                    }
                    FlowControlCommand::HoveredClosed => {
                        nesting_level -= 1;

                        if let Some(skip_level) = skip {
                            if skip_level == nesting_level{
                                skip = None;
                            }
                        }
                    }
                    FlowControlCommand::ClickedOpened { event } => {
                        if skip.is_none() {
                            skip = Some(nesting_level);

                            if layout_engine.hovered() {
                                if clicked {
                                    skip = None;
                                    if let Some(event) = event {
                                        match locals {
                                            None => events.push(match Event::from_str(event) {
                                                Err(_) => Event::default(),
                                                Ok(event) => event
                                            }),
                                            Some(locals) => {
                                                match locals.get(event) {
                                                    None => events.push(match Event::from_str(event) {
                                                        Err(_) => Event::default(),
                                                        Ok(event) => event
                                                    }),
                                                    Some(command) => {
                                                        match command {
                                                            PageDataCommand::GetEvent { local:_, from } => {
                                                                if let Some(event) = user_app.get_event(&from, &list_data) {
                                                                    events.push(event);
                                                                }
                                                            }
                                                            PageDataCommand::SetEvent { local:_, to } => {
                                                                events.push(to.clone());
                                                            }
                                                            _ => events.push(match Event::from_str(event) {
                                                                Err(_) => Event::default(),
                                                                Ok(event) => event
                                                            }),
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        nesting_level += 1;
                    }
                    FlowControlCommand::ClickedClosed => {
                        nesting_level -= 1;

                        if let Some(skip_level) = skip {
                            if skip_level == nesting_level{
                                skip = None;
                            }
                        }
                    }
                    FlowControlCommand::ListOpened { src } => {
                        nesting_level += 1;

                        if skip.is_none() {
                            recursive_source = src.to_string();
                            recursive_commands.clear();
                            recursive_call_stack.clear();
                            collect_list_commands = true;
                            collect_recursive_declarations = true;
                        }
                        
                    }
                    FlowControlCommand::ListClosed => {
                        nesting_level -= 1;

                        if skip.is_none(){
                            let list_length = user_app.get_list_length(&recursive_source, &None);
                            
                            if let Some(source) = list_length {
                                for i in 0..source {
                                    set_layout(
                                        clicked,
                                        events,
                                        &recursive_commands, 
                                        reusables,
                                        Some(&recursive_call_stack), 
                                        Some(ListData { src: &recursive_source, index: i }), 
                                        &mut None, 
                                        &mut None, 
                                        layout_engine, 
                                        user_app
                                    );
                                }
                            }
                        }
                    }
                    FlowControlCommand::ElementOpened { id:_ } => {
                        nesting_level += 1;

                        if skip.is_none() {
                            layout_engine.open_element();
                        }
                    }
                    FlowControlCommand::ElementClosed => {
                        nesting_level -= 1;

                        if skip.is_none() {
                            layout_engine.close_element();
                        }
                    }
                    FlowControlCommand::ConfigOpened => {
                        nesting_level += 1;
        
                        if skip.is_none() {
                            config = Some(ElementConfiguration::default());
                        }
                    }
                    FlowControlCommand::ConfigClosed => {
                        nesting_level -= 1;
        
                        if skip.is_none() && append_config.is_none(){
                            let final_config = config.take().unwrap();
                            layout_engine.configure_element(&final_config);
                        }
                        else {
                            //println!("config actually not closed");
                        }
                    }
                    FlowControlCommand::UseOpened { name } => {
                        nesting_level += 1;

                        if skip.is_none() {
                            recursive_commands.clear();
                            recursive_call_stack.clear();
                            collect_recursive_declarations = true;
                            recursive_source = name.to_string();
                        }
                        
                    }
                    FlowControlCommand::UseClosed => {
                        nesting_level -= 1;

                        if skip.is_none() {
                            collect_recursive_declarations = false;
                            if let Some(reusable) = reusables.get(&recursive_source){
                                for command in reusable.iter() {
                                    recursive_commands.push(command);
                                }
                                if recursive_call_stack.len() > 0 {
                                    set_layout(clicked, events, &recursive_commands, reusables, Some(&recursive_call_stack), None, &mut config, &mut text_config, layout_engine, user_app);
                                }
                                else {
                                    set_layout(clicked, events, &recursive_commands, reusables, None, None, &mut config, &mut text_config, layout_engine, user_app);
                                }
                            }
                            
                        }
                    }
                    FlowControlCommand::TextElementOpened => {
                        nesting_level += 1;

                        if skip.is_none() {
                            text_config = Some(TextConfig::default());
                        }
                    }
                    FlowControlCommand::TextElementClosed => {
                        if skip.is_none() {
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
                        }
        
                        nesting_level -= 1;
                    }
                    FlowControlCommand::TextConfigOpened => {
                        nesting_level += 1;
                    }
                    FlowControlCommand::TextConfigClosed => nesting_level -= 1,
                }
            }
            LayoutCommandType::PageData(data_command) => {
                if collect_recursive_declarations {
                    let name = data_command.get_local();
                    recursive_call_stack.insert(name, data_command);
                }
            }
            LayoutCommandType::ElementConfig(config_command) => {
                if skip.is_none() {
                    let open_config = match append_config.is_some() {
                        true => append_config.as_mut().unwrap(),
                        false => config.as_mut().unwrap()
                    };
                    match config_command {
                        ConfigCommand::FitX  => open_config.x_fit().parse(),
                        ConfigCommand::FitXmin{min}  => open_config.x_fit_min(*min).parse(),
                        ConfigCommand::FitXminmax{min, max}  => open_config.x_fit_min_max(*min, *max).parse(),
                        ConfigCommand::FitY  => open_config.y_fit().parse(),
                        ConfigCommand::FitYmin{min}  => open_config.y_fit_min(*min).parse(),
                        ConfigCommand::FitYminmax{min, max}  => open_config.y_fit_min_max(*min, *max).parse(),
                        ConfigCommand::GrowX  => open_config.x_grow().parse(),
                        ConfigCommand::GrowXmin{min} => open_config.x_grow_min(*min).parse(),
                        ConfigCommand::GrowXminmax{min, max}  => open_config.x_grow_min_max(*min, *max).parse(),
                        ConfigCommand::GrowY  => open_config.y_grow().parse(),
                        ConfigCommand::GrowYmin{min} => open_config.y_grow_min(*min).parse(),
                        ConfigCommand::GrowYminmax{min, max}  => open_config.y_grow_min_max(*min, *max).parse(),
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
                        ConfigCommand::DynamicId(name) => {
                            if let Some(locals) = locals {
                                if let Some(data_command) = locals.get(name) {
                                    if let PageDataCommand::SetText { local:_, to } = data_command {
                                        open_config.id(&to);
                                    }
                                }
                            }
                        }
                        ConfigCommand::StaticId(label)|
                        ConfigCommand::Id(label)  => open_config.id(&label).parse(),
                        ConfigCommand::ChildGap(gap)  => open_config.child_gap(*gap).parse(),
                        ConfigCommand::ChildAlignmentXLeft  => open_config.align_children_x_left().parse(),
                        ConfigCommand::ChildAlignmentXRight  => open_config.align_children_x_right().parse(),
                        ConfigCommand::ChildAlignmentXCenter  => open_config.align_children_x_center().parse(),
                        ConfigCommand::ChildAlignmentYTop  => open_config.align_children_y_top().parse(),
                        ConfigCommand::ChildAlignmentYCenter  => open_config.align_children_y_center().parse(),
                        ConfigCommand::ChildAlignmentYBottom  => open_config.align_children_y_bottom().parse(),
                        ConfigCommand::Color(color)  => open_config.color(*color).parse(),
                        ConfigCommand::DynamicColor(color) => match locals {
                            None => match user_app.get_color(color, &list_data) {
                                None => open_config.color(Color::default()).parse(),
                                Some(color) => open_config.color(*color).parse(),
                            }
                            Some(locals) =>  match locals.get(color) {
                                None => match user_app.get_color(color, &list_data) {
                                    None => open_config.color(Color::default()).parse(),
                                    Some(color) => open_config.color(*color).parse(),
                                }
                                Some(data_command) => {
                                    match data_command {
                                        PageDataCommand::GetColor { local:_, from } =>  match user_app.get_color(from, &list_data) {
                                            None => open_config.color(Color::default()).parse(),
                                            Some(color) => open_config.color(*color).parse(),
                                        }
                                        PageDataCommand::SetColor { local:_, to } => open_config.color(*to).parse(),
                                        _ => open_config.color(Color::default()).parse(),
                                    }
                                }
                            }
                        }
                        ConfigCommand::RadiusAll(radius)  => open_config.radius_all(*radius).parse(),
                        ConfigCommand::RadiusTopLeft(radius)  => open_config.radius_top_left(*radius).parse(),
                        ConfigCommand::RadiusTopRight(radius)  => open_config.radius_bottom_right(*radius).parse(),
                        ConfigCommand::RadiusBottomRight(radius)  => open_config.radius_bottom_right(*radius).parse(),
                        ConfigCommand::RadiusBottomLeft(radius)  => open_config.radius_bottom_left(*radius).parse(),
                        ConfigCommand::BorderColor(color) => open_config.border_color(*color).parse(),
                        ConfigCommand::BorderDynamicColor(color) => match locals {
                            None => match user_app.get_color(color, &list_data) {
                                None => open_config.border_color(Color::default()).parse(),
                                Some(color) => open_config.border_color(*color).parse(),
                            }
                            Some(locals) =>  match locals.get(color) {
                                None => match user_app.get_color(color, &list_data) {
                                    None => open_config.border_color(Color::default()).parse(),
                                    Some(color) => open_config.border_color(*color).parse(),
                                }
                                Some(data_command) => {
                                    match data_command {
                                        PageDataCommand::GetColor { local:_, from } =>  match user_app.get_color(from, &list_data) {
                                            None => open_config.border_color(Color::default()).parse(),
                                            Some(color) => open_config.border_color(*color).parse(),
                                        }
                                        PageDataCommand::SetColor { local:_, to } => open_config.border_color(*to).parse(),
                                        _ => open_config.border_color(Color::default()).parse(),
                                    }
                                }
                            }
                        }
                        ConfigCommand::BorderAll(border)  => open_config.border_all(*border as u16).parse(),
                        ConfigCommand::BorderTop(border)  => open_config.border_top(*border as u16).parse(),
                        ConfigCommand::BorderBottom(border)  => open_config.border_bottom(*border as u16).parse(),
                        ConfigCommand::BorderLeft(border)  => open_config.border_left(*border as u16).parse(),
                        ConfigCommand::BorderRight(border)  => open_config.border_right(*border as u16).parse(),
                        ConfigCommand::BorderBetweenChildren(border)  => open_config.border_between_children(*border as u16).parse(),
                        ConfigCommand::Clip { vertical, horizontal } => {
                            let child_offset = layout_engine.get_scroll_offset();
                            open_config.scroll(*vertical, *horizontal, child_offset).parse()
                        }
                        ConfigCommand::Image { name } => {
                            match locals {
                                None => match user_app.get_image(name, &list_data) {
                                    None => {},
                                    Some(image) => open_config.image(image).parse(),
                                }
                                Some(locals) =>  match locals.get(name) {
                                    None => match user_app.get_image(name, &list_data) {
                                        None => {},
                                        Some(image) => open_config.image(image).parse(),
                                    }
                                    Some(data_command) => {
                                        match data_command {
                                            PageDataCommand::GetImage { local:_, from } =>  match user_app.get_image(from, &list_data) {
                                                None => {},
                                                Some(image) => open_config.image(image).parse(),
                                            }
                                            _ => {},
                                        }
                                    }
                                }
                            }
                        }
                        ConfigCommand::Floating => open_config.floating().parse(),
                        ConfigCommand::FloatingOffset { x, y } => open_config.floating_offset(*x, *y).parse(),
                        ConfigCommand::FloatingDimensions { width, height } => open_config.floating_dimensions(*width, *height).parse(),
                        ConfigCommand::FloatingZIndex { z } => open_config.floating_z_index(*z).parse(),
                        ConfigCommand::FloatingAttatchToParentAtTopLeft => open_config.floating_attach_to_parent_at_top_left().parse(),
                        ConfigCommand::FloatingAttatchToParentAtCenterLeft => open_config.floating_attach_to_parent_at_center_left().parse(),
                        ConfigCommand::FloatingAttatchToParentAtBottomLeft => open_config.floating_attach_to_parent_at_bottom_left().parse(),
                        ConfigCommand::FloatingAttatchToParentAtTopCenter => open_config.floating_attach_to_parent_at_top_center().parse(),
                        ConfigCommand::FloatingAttatchToParentAtCenter => open_config.floating_attach_to_parent_at_center().parse(),
                        ConfigCommand::FloatingAttatchToParentAtBottomCenter => open_config.floating_attach_to_parent_at_bottom_center().parse(),
                        ConfigCommand::FloatingAttatchToParentAtTopRight => open_config.floating_attach_to_parent_at_top_right().parse(),
                        ConfigCommand::FloatingAttatchToParentAtCenterRight => open_config.floating_attach_to_parent_at_center_right().parse(),
                        ConfigCommand::FloatingAttatchToParentAtBottomRight => open_config.floating_attach_to_parent_at_bottom_right().parse(),
                        ConfigCommand::FloatingAttatchElementAtTopLeft => open_config.floating_attach_element_at_top_left().parse(),
                        ConfigCommand::FloatingAttatchElementAtCenterLeft => open_config.floating_attach_element_at_center_left().parse(),
                        ConfigCommand::FloatingAttatchElementAtBottomLeft => open_config.floating_attach_element_at_bottom_left().parse(),
                        ConfigCommand::FloatingAttatchElementAtTopCenter => open_config.floating_attach_element_at_top_center().parse(),
                        ConfigCommand::FloatingAttatchElementAtCenter => open_config.floating_attach_element_at_center().parse(),
                        ConfigCommand::FloatingAttatchElementAtBottomCenter => open_config.floating_attach_element_at_bottom_center().parse(),
                        ConfigCommand::FloatingAttatchElementAtTopRight => open_config.floating_attach_element_at_top_right().parse(),
                        ConfigCommand::FloatingAttatchElementAtCenterRight => open_config.floating_attach_element_at_center_right().parse(),
                        ConfigCommand::FloatingAttatchElementAtBottomRight => open_config.floating_attach_element_at_bottom_right().parse(),
                        ConfigCommand::FloatingPointerPassThrough => open_config.floating_pointer_pass_through().parse(),
                        ConfigCommand::FloatingAttachElementToElement { other_element_id:_ } => {
                            //let id = layout_engine.get_id(other_element_id);
                            open_config.floating_attach_to_element(0).parse()
                        }
                        ConfigCommand::FloatingAttachElementToRoot => open_config.floating_attach_to_root().parse(),
                    }
                }
            }
            LayoutCommandType::TextConfig(config_command) => {
                if skip.is_none() {
                    let text_config = text_config.as_mut().unwrap();
                    match config_command {
                        TextConfigCommand::AlignCenter => text_config.alignment_center().parse(),
                        TextConfigCommand::AlignLeft => text_config.alignment_left().parse(),
                        TextConfigCommand::AlignRight => text_config.alignment_right().parse(),
                        TextConfigCommand::Color(color) => text_config.color(*color).parse(),
                        TextConfigCommand::Editable(_state) => (),
                        TextConfigCommand::Content(content) => text_content = Some(content),
                        TextConfigCommand::DefaultText(_default) => {}
                        TextConfigCommand::DynamicContent(name) => {
                            #[cfg(feature="parse_logger")]
                            println!("-------------------Command: Dynamic Text Content. Name: {:?}", name);
                            match locals {
                                None => match user_app.get_text(name, &list_data) {
                                    None => {
                                        text_content = None;
                                        dynamic_text_content = None;
                                    }
                                    Some(text) => dynamic_text_content = Some(text),
                                }
                                Some(locals) =>  match locals.get(name) {
                                    None => match user_app.get_text(name, &list_data) {
                                        None => {
                                            text_content = None;
                                            dynamic_text_content = None;
                                        }
                                        Some(text) => dynamic_text_content = Some(text),
                                    }
                                    Some(data_command) => {
                                        #[cfg(feature="parse_logger")]
                                        println!("trying to get dynamic text: {:?}", name);
                                        match data_command {
                                            PageDataCommand::SetText { local:_, to } => {
                                                dynamic_text_content = Some(to);
                                            }
                                            PageDataCommand::GetText { local:_, from } => {
                                                match user_app.get_text(from, &list_data) {
                                                    None => {
                                                        text_content = None;
                                                        dynamic_text_content = None;
                                                    }
                                                    Some(text) => dynamic_text_content = Some(text),
                                                }
                                            }
                                            _ => {
                                                text_content = None;
                                                dynamic_text_content = None;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        TextConfigCommand::FontId(id) => text_config.font_id(*id).parse(),
                        TextConfigCommand::FontSize(size) => text_config.font_size(*size).parse(),
                        TextConfigCommand::LineHeight(height) => text_config.line_height(*height).parse(),
                    }
                }
                
            }
        }
    }
}
