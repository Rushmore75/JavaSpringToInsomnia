#![feature(pattern)]
#![feature(iter_array_chunks)]
#![feature(mem_copy_fn)]
#![feature(layout_for_ptr)]

use std::fs;
use java_annotations::parse_annotations;

use crate::exports::{workspace::Workspace, base::Base, request::Request, environment::Environment};

mod exports;
mod java_annotations;

fn main() {

    // create the workspace and the base together
    let workspace = Workspace::new("Workspace Name", "");
    let mut base = Base::new(workspace);
    
    // add 0 or more methods

    // add 0 or more environments
    let mut sub_env = Environment::new_sub_environment(base.get_base_environment_id(), "Sub Code Environment");
    
    // TODO change the file input to cmd line input
    let mut file = fs::read_to_string("example.java").unwrap();
    let find_annotation = "@RequestMapping";

    
    // Parse out annotations, with names, data
    let data = parse_annotations(&mut file, find_annotation);
    // Go thru information
    data.iter().for_each(|f| {
        match f {
            Some(thing) => {
                
                // Look for "method" magic value so it can be added to Insomnia
                match &thing.annotation_data.get("method") {
                    Some(method) => {


                        let value = **&thing.annotation_data.get("value").unwrap();

                        // 4 curly brackets because half of them are escaping.  // get rid of quotes
                        let url = format!{"{{{{ _.baseurl }}}}{}", java_annotations::get_slice_inside("\"", "\"", value)};

                        base.add_method(Request::new(
                            base.get_workspace_id(),
                            url,
                            &thing.target_name.name,
                            "",
                            method
                        ));
                        
                            
                    },
                    None => {},
                }
                // Blank string is used for a string value in the annotation
                match &thing.annotation_data.get("") {
                    Some(x) => {
                        sub_env.add_environment_data("baseurl", x);    
                    },
                    None => {},
                }
            },
            None => {/* that's unfortunate */},
        }
    });
    
    base.add_environment(sub_env);
    // convert into json
    let v = serde_json::to_string(&base).unwrap();
    fs::write("output.json", v).unwrap();


}

#[cfg(test)]
mod tests {
    use crate::java_annotations;


    #[test]
    fn remove_inside_quotes() {
        let case = "Jazz\"case\"Jazz";
        let inside = java_annotations::remove_inside("\"", "\"", case);
        assert_eq!(inside, "JazzJazz");
    }

    #[test]
    fn remove_inside_line_comment() {
        let inside = java_annotations::remove_inside("/*", "*/", "This |/*case*/| This");
        assert_eq!(inside, "This || This");
    }

    #[test]
    fn slice_inside_quotes() {
        let inside = java_annotations::get_slice_inside("\"", "\"", "Jazz\"case\"Jazz");
        assert_eq!(inside, "case");
    }

    #[test]
    fn slice_inside_comment() {
        let inside = java_annotations::get_slice_inside("/*", "*/", "This |/*case*/| This");
        assert_eq!(inside, "case");
        // left: `"*case"`,
        // right: `"case"`'
    }


}
