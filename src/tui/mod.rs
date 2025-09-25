// tui
//
use crossterm::{self};

use ratatui::{self,
        DefaultTerminal,
        Frame,
        widgets::{Widget,Block,List,Paragraph},
        text::{Line,Span},
        layout::Rect,
        prelude::{Constraint,Direction,Layout},
        style::{Modifier,self,Style,Color},
    };
use std::collections::BTreeMap;
use std::io;


pub enum FocusedField{
    Directory,
    MiniDirectory,
    MiniExtension,
    LwrMiniButton,
    UpprMiniButton,
}

pub struct App {

    pub monitoring_dir: String,
    pub dir_buffer: String,
    pub ext_buffer: String,
    pub file_dir_map:  BTreeMap<String,Vec<String>>,

    pub focused_field: FocusedField,
    pub exit: bool,
}

impl App {
     fn draw(&mut self,frame: &mut Frame){
        frame.render_widget(self,frame.area());
    }
     pub fn run(&mut self,terminal: &mut ratatui::DefaultTerminal) -> io::Result<()>{

        loop{

            terminal.draw(|frame| self.draw(frame))?;
            if let Ok(_) = self.handle_event() {
                break Ok(());
            }

        }
    }
     fn handle_event(&mut self)->io::Result<()>{
        match crossterm::event::read()? {
            crossterm::event::Event::Key(key_event) if key_event.kind == crossterm::event::KeyEventKind::Press =>{
                self.handle_key_event(key_event);
            }
            _=>{
                //Don't require any more bindings (as of now)
            }
        }
        Ok(())
    }
     fn handle_key_event(&mut self,key_event: crossterm::event::KeyEvent){
        match key_event.code  {
                crossterm::event::KeyCode::Esc =>{
                    self.exit = true;
                } 
                crossterm::event::KeyCode::Tab =>{
                    match self.focused_field {

                        //circle of enum for focus
                        FocusedField::Directory => {
                            self.focused_field = FocusedField::MiniDirectory;
                        }
                        FocusedField::MiniDirectory => {
                            self.focused_field = FocusedField::MiniExtension;
                        }
                        FocusedField::MiniExtension => {
                            self.focused_field = FocusedField::UpprMiniButton;
                        }
                        FocusedField::UpprMiniButton => {
                            self.focused_field = FocusedField::LwrMiniButton;
                        }
                        FocusedField::LwrMiniButton => {
                            self.focused_field = FocusedField::Directory;
                        }

                    }
                } 
                key_code => {
                    self.handle_user_typing(key_code);
                }
        }
    }
     fn handle_user_typing(&mut self,key_code:crossterm::event::KeyCode ){

        match self.focused_field {
            FocusedField::Directory => {
                match key_code{
                    crossterm::event::KeyCode::Char(c)=>{
                        self.monitoring_dir = format!("{}{}",self.monitoring_dir,c);
                    }
                    crossterm::event::KeyCode::Backspace=> {
                        self.monitoring_dir.pop();
                    }
                    _=>{ 
                        //Don't require any more bindings (as of now)
                    }
                }
            }
            FocusedField::MiniDirectory => {
                match key_code{
                    crossterm::event::KeyCode::Char(c)=>{
                        self.dir_buffer= format!("{}{}",self.dir_buffer,c);
                    }
                    crossterm::event::KeyCode::Backspace=> {
                        self.dir_buffer.pop();
                    }
                    _=>{ 
                        //Don't require any more bindings (as of now)
                    }
                }
            }
            FocusedField::MiniExtension => {
                match key_code{
                    crossterm::event::KeyCode::Char(c)=>{
                        self.ext_buffer = format!("{}{}",self.ext_buffer,c);
                    }
                    crossterm::event::KeyCode::Backspace=> {
                        self.ext_buffer.pop();
                    }
                    _=>{ 
                        //Don't require any more bindings (as of now)
                    }
                }
            }
            FocusedField::UpprMiniButton => {
                match key_code{
                    crossterm::event::KeyCode::Enter=> {
                        self.emit_type_extns();
                    }
                    _=>{ 
                        //Don't require any more bindings (as of now)
                    }
                }
            }
            FocusedField::LwrMiniButton => {
                match key_code{
                    crossterm::event::KeyCode::Enter=> {
                        let _ = self.file_dir_map.pop_last();
                    }
                    _=>{ 
                        //Don't require any more bindings (as of now)
                    }
                }
            }

        }
    }
    
    // emit the buffers to the actual hashmap
     fn emit_type_extns(&mut self){
        let transformed_buffer : Vec<String> = self.ext_buffer.split(",").map(|e| e.to_string()).collect();
        self.file_dir_map.insert(self.dir_buffer.clone(),transformed_buffer);
    }

}

impl Widget for &mut App {
     fn render(self,area: Rect,buf: &mut ratatui::buffer::Buffer)->(){

        let mut list_items :Vec<String> = vec![];
        let dir_ext_vec: Vec<_> = self.file_dir_map.clone().into_iter().collect::<Vec<_>>();

        //styles
        //

        let mut monitoring_dir_field_border_focus = ratatui::widgets::BorderType::Plain;
        let mut mext_field_border_focus = ratatui::widgets::BorderType::Plain;
        let mut mtype_field_border_focus = ratatui::widgets::BorderType::Plain;
        let mut l_mbutton_border_focus = ratatui::widgets::BorderType::Plain;
        let mut u_mbutton_border_focus = ratatui::widgets::BorderType::Plain;

        match self.focused_field {
            FocusedField::Directory=>{
                monitoring_dir_field_border_focus = ratatui::widgets::BorderType::Double;
            }
            FocusedField::MiniExtension=>{
                mext_field_border_focus = ratatui::widgets::BorderType::Double;
            }
            FocusedField::MiniDirectory=>{
                mtype_field_border_focus = ratatui::widgets::BorderType::Double;
            }
            FocusedField::LwrMiniButton=>{
                l_mbutton_border_focus = ratatui::widgets::BorderType::Double;
            }
            FocusedField::UpprMiniButton=>{
                u_mbutton_border_focus = ratatui::widgets::BorderType::Double;
            }
        }

        // titles
        //
        let main_title = Line::from (Span::styled("  DIRMON  ",Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Green)))
            .centered();

        let inp_field_monitoring_dir_title = Line::from (Span::styled(" Monitoring Directory ",Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Green)))
            .centered();

        let inp_mfield_extns_title = Line::from (Span::styled(" Extensions(, seperated) ",Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Green)))
                .centered();

        let inp_mfield_types_title = Line::from (Span::styled(" Directory ",Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Green)))
            .centered();
        let list_title = Line::from (Span::styled(" List ",Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Green)))
            .centered();




        // monitoring-dir 
        let monitoring_dir= Paragraph::new(self.monitoring_dir.clone())
            .block(Block::new()
                .borders(ratatui::widgets::Borders::ALL)
                .border_type(monitoring_dir_field_border_focus)
                .title(inp_field_monitoring_dir_title))
            .centered();

        // block 
        let main_block = Block::bordered()
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .title(main_title);


        // directory-extension lists
        for (key,val) in dir_ext_vec {
            let extension_string = val.join(",");
            list_items.push(format!("{} {}",key,extension_string));
        }
        let list = List::new(list_items)
                    .block(Block::new()
                        .borders(ratatui::widgets::Borders::ALL)
                        .title(list_title)
                        .border_type(ratatui::widgets::BorderType::Rounded))
                    .highlight_symbol(">>>");

        // I/O
        // change directory field
        
        // field 
        let lower_inner_block = Block::bordered()
                    .border_type(ratatui::widgets::BorderType::Rounded);

        let mtype_field = Paragraph::new(self.dir_buffer.clone())
            .block(Block::new()
                .borders(ratatui::widgets::Borders::ALL)
                .border_type(mtype_field_border_focus)
                .title(inp_mfield_types_title))
            .centered();

        let mext_field = Paragraph::new(self.ext_buffer.clone())
            .block(Block::new()
                .borders(ratatui::widgets::Borders::ALL)
                .border_type(mext_field_border_focus)
                .title(inp_mfield_extns_title))
            .centered();

        let l_mbutton = Paragraph::new("Remove".to_string())
            .block(Block::new()
                .borders(ratatui::widgets::Borders::ALL)
                .border_type(l_mbutton_border_focus))
            .centered();
        let u_mbutton = Paragraph::new("Add".to_string())
            .block(Block::new()
                .borders(ratatui::widgets::Borders::ALL)
                .border_type(u_mbutton_border_focus))
            .centered();

        //finalizing block
        let inner_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Percentage(25), // monitoring directory
                Constraint::Percentage(50), // type-extension binding and buttons
                Constraint::Percentage(25), // edit directory 
            ])
            .split(main_block.inner(area));
        let middle_inner_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([
                Constraint::Percentage(70), // type-extension binding
                Constraint::Percentage(30), // buttons
            ])
            .split(inner_chunks[1]);



        let lower_inner_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([
                Constraint::Percentage(40), // monitoring directory
                Constraint::Percentage(60), // type-extension binding 
            ])
            .split(inner_chunks[2]);

        let middle_inner_button_chunks= Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(50), // top button 
                Constraint::Percentage(50), // bottom button
            ])
            .split(middle_inner_chunks[1]);

        monitoring_dir.render(inner_chunks[0],buf);
        mtype_field.render(lower_inner_chunks[0],buf);
        mext_field.render(lower_inner_chunks[1],buf);

        l_mbutton.render(middle_inner_button_chunks[1],buf);
        u_mbutton.render(middle_inner_button_chunks[0],buf);


        list.render(inner_chunks[1], buf);
        main_block.render(area,buf);

    }
}

impl Default for App {
    fn default()->Self{
        Self{
            monitoring_dir: "/".to_string(),
            dir_buffer:"".to_string(),
            ext_buffer:"".to_string(),

            file_dir_map:BTreeMap::from([("Audio".to_string(),vec!["mp3".to_string(),"wav".to_string()])]),
            focused_field: FocusedField::Directory,
            exit: false,
        }
    }
}
