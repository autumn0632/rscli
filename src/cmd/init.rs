use inquire::{
    ui::{RenderConfig, Styled},
    Confirm, Select, Text,
};

use crate::utils::{check_ip, check_ip_cidr};

#[derive(Debug)]
pub struct Init {
    interfaces: Vec<String>,
    choice_inter: String,
    default_route: String,
    ipv4: String,
    ipv6: String,
    render_config: RenderConfig,
}

impl Init {
    pub fn new() -> Self {
        Init {
            interfaces: Vec::new(),
            choice_inter: String::new(),
            default_route: String::new(),
            ipv4: String::new(),
            ipv6: String::new(),
            render_config: RenderConfig::default(),
        }
    }

    pub fn running(&mut self) {
        self.set_render_config();
        self.get_default_route();
        self.get_interface();
        self.print_prefix();
        self.select_interface();
        self.input_ipv4();
        self.input_ipv6();
        self.print();
    }

    fn set_render_config(&mut self) {
        self.render_config.prompt_prefix = Styled::new("");
        self.render_config.answered_prompt_prefix = Styled::new("");
    }

    fn print_prefix(&mut self) {
        println!("Noticd:");
        println!("    This device has not been initialized. ");
        println!("    This wizard will help you configure the management network interface and required system settings.");
        println!();
    }

    fn select_interface(&mut self) {
        let sel = Select::new("Select the interface to configure", self.interfaces.clone())
            .with_render_config(self.render_config)
            .prompt();
        match sel {
            Ok(inter) => {
                println!("You choice: {}", inter);
                self.choice_inter = inter;
            }
            Err(err) => println!("{}", err),
        }
    }

    fn get_interface(&mut self) {
        self.interfaces = vec![String::from("eth0"), String::from("eth1")]
    }

    fn get_default_route(&mut self) {}

    fn input_ipv4(&mut self) {
        let confir = Confirm::new("COnfig IPv4?")
            .with_render_config(self.render_config)
            .prompt();

        match confir {
            Ok(ans) => {
                if ans {
                    self.input_ip("1.1.1.1/24".into());
                    self.input_default_route("");
                }
            }
            Err(err) => println!("{}", err),
        }
    }

    fn input_ipv6(&mut self) {
        let confir = Confirm::new("COnfig IPv6?")
            .with_render_config(self.render_config)
            .prompt();

        match confir {
            Ok(ans) => {
                if ans {
                    self.input_ip("".into());
                    self.input_default_route("");
                }
            }
            Err(err) => println!("{}", err),
        }
    }

    fn input_ip(&mut self, def_ip: &str) {
        loop {
            let input_ip;
            if def_ip.is_empty() {
                input_ip = Text::new("IP/MASK")
                    .with_render_config(self.render_config)
                    .with_default(def_ip)
                    .prompt();
            } else {
                input_ip = Text::new("IP/MASK")
                    .with_render_config(self.render_config)
                    .prompt();
            }

            match input_ip {
                Ok(ip) => match check_ip_cidr(&ip) {
                    Ok(ip) => {
                        if ip.is_ipv4() {
                            self.ipv4 = ip.to_string();
                        } else {
                            self.ipv6 = ip.to_string();
                        }
                        break;
                    }
                    Err(err) => println!("{}", err),
                },
                Err(err) => println!("{}", err),
            }
        }
    }

    fn input_default_route(&mut self, old_route: &str) {
        if !self.default_route.is_empty() {
            return;
        }

        loop {
            let input_route;
            if old_route.is_empty() {
                input_route = Text::new("Gateway Address")
                    .with_render_config(self.render_config)
                    .with_default("")
                    .prompt();
            } else {
                input_route = Text::new("Gateway Address")
                    .with_render_config(self.render_config)
                    .with_default(old_route)
                    .prompt();
            }

            match input_route {
                Ok(ref route) => match check_ip(&route) {
                    Ok(r) => {
                        self.default_route = r.to_string();
                        break;
                    }
                    Err(err) => println!("{}", err),
                },
                Err(ref err) => println!("{}", err),
            }
        }
    }
    fn print(&self) {
        println!("Your choice:");
        println!("    Interface: {}", self.choice_inter);
        println!("    IPv4: {}", self.ipv4);
        println!("    IPv6: {}", self.ipv6);
        println!("    Default Route: {}", self.default_route);
    }
}
