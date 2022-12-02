//=========================================================================//
/*                   Parse the .java file for annotations                  */
//=========================================================================//

use std::collections::HashMap;
// TODO these two methods should have a common parent that divides a string into three

/// can take data out from quotes, etc
pub fn get_slice_inside<'a>(opener: &'a str, closer: &'a str, string: &'a str) -> &'a str {
    let len = closer.len();
    
    let start = string.find(opener).unwrap();
    // make sure not to select the same pattern if both the opener and closer are the same
    let end = string[start+1..].find(closer).unwrap() + start; // add back the start offset
    
    &string[start+len..=end]
}

/// operates on whole string, removes only the first occurrence
pub fn remove_inside<'b>(opener: &'b str, closer: &'b str, input: &'b str) -> String {
    let len = closer.len();

    let start = input.find(opener).unwrap();
    let end = input[start+1..].find(closer).unwrap() + start; // don't select the same Pattern twice if the same
    
    let one = input.get(..start).unwrap();
    let two = input.get(end+len+1..).unwrap();
    
    one.to_owned() + two
}





/// operates on a single line
fn remove_full_line_comment<'b>(line: &'b str) -> &'b str {
    if line.contains("//") {
        let beginning = line.find("//").unwrap();
        // slice of everything before the comment
        return &line[..beginning];
    }
    line
}

/// goes through input string and looks for, and removes, comments.
/// It supports both
/// ```
/// // full line comments
/// /* and inline comments */
/// ```
fn remove_comments<'a>(input: &'a mut str) -> &'a str {
    let mut product = input.to_string();
    
    // remove full line comments
    product = product.lines().fold(String::new(), |fold, f| {
        fold + remove_full_line_comment(f) + "\n"
    });

    // test for and remove inline comments
    while product.contains("/*") || product.contains("*/") {
        product = remove_inside("/*", "*/", &product);
    }

    // FIXME leaves single / in lots of places
    
    unsafe {
        // product is shorter, use it as the length, count it just how many to swap, so swapping the the longer would be bad.
        std::ptr::swap_nonoverlapping(
            input.as_bytes_mut().as_mut_ptr(),
            product.as_bytes_mut().as_mut_ptr(),
            product.len()
        );
    };
    // get rid of extra data
    &input[..=product.len()]
    
}

pub fn parse_annotations<'a>(file: &'a mut str, find_annotation: &'a str) -> Vec<Option<JavaAnnotation<'a>>> {

    // TODO split up into generalized methods

    let no_comments_file = remove_comments(file);

    let mut results: Vec<Option<JavaAnnotation>> = Vec::new();

    // find all lines that have the given annotation on them
    no_comments_file
        .lines()
        .enumerate()
        // find annotations
        .filter(|f| f.1.contains(find_annotation))
        // loop thru annotations
        .for_each(|(line_number, line)| {
            // not going to worry about inline annotations, they are dumb :P
            
            // take original file again
            let name_line = no_comments_file
                .lines()
                // Jump to annotation
                .skip(line_number)
                // Skip other potential annotations
                .skip_while(|x| x.trim_start().starts_with('@')) 
                // TODO this might take the wrong line, may need to back up one
                // Take the next line (the name is probably on it)
                .take(1)
                // Collect as String
                .collect::<String>();

            // Obtain the name from the line we think it's on
            let name = parse_name(&name_line);

            // Add to our results the:
            results.push(match name {
                Some(java_name) => {
                    
                    // Obtain the information from inside the annotation
                    // @Foo(this stuff here)
                    let data = get_slice_inside("(", ")", line);

                    // Create map to hold the data
                    let mut annotation_data = HashMap::new();
                    // If the data is just a string
                    // @Foo("example")
                    if data.starts_with('"') && data.ends_with('"') {                    
                        // TODO it still could be two strings? Is that allowed?
                        annotation_data.insert("", get_slice_inside("\"", "\"", data));
                    }
                    // It must have map-able data inside
                    // @Foo(value="/api/", method=GET)
                    else {
                        let splits = data.split(',');
                        splits.for_each(|f| {
                            // Remove whitespace, then split at equals,
                            // enter these values as a Key / Value pair
                            let x = f.trim().split('=').collect::<Vec<&str>>();
                            annotation_data.insert(x[0], x[1]);
                        })
                    }
                    // return our parsed data
                    Some(JavaAnnotation {
                            target_name: java_name,
                            annotation_data,
                    })
                }
                None => None,
            })
        });
        
    results

}

/// Input is the line you think there might be a name on.
fn parse_name(name_line: &str) -> Option<JavaName> {

    if name_line.contains(" class ") {
        // class annotation
        
        // get first work after "class"
        let name = name_line
            .split_whitespace()
            .skip_while(|f| f != &"class")
            .skip(1)
            .take(1)
            .collect::<String>();

        println!("{name}");
        
        return Some(JavaName {
            name,
            part: JavaType::Class
        });
        
    } else if name_line.contains('=') {
        // variable annotation
        
        // get the word just before equals sign
        let name = name_line
            .split('=')
            .collect::<Vec<&str>>()
            [0]
            .chars()
            .rev()
            .skip_while(|f| f == &' ') // skip preceding whitespace
            .take_while(|f| f != &' ') // go until more whitespace
            .collect::<String>()
            .chars()
            .rev()
            .collect::<String>();
        
            println!("{name}");

            return Some(JavaName {
                name,
                part: JavaType::Variable
            })

    } else if name_line.contains('(') {
        // it's probably a method annotation
        let name = name_line
            .split('(')
            .collect::<Vec<&str>>()
            [0]
            .chars()
            .rev()
            .take_while(|f| f != &' ')
            .collect::<String>()
            .chars()
            .rev()
            .collect::<String>();
            
        println!("{name}");

        return Some(JavaName {
            name,
            part: JavaType::Method
        })
    }

    None
}

pub struct JavaAnnotation<'a> {
    pub target_name: JavaName,
    // Unfortunately this has to use String instead of &str, just a
    // little less fun is all. But considering it actually compiles
    // this way it's better.
    pub annotation_data: HashMap<&'a str, &'a str>,
}
    
pub struct JavaName {
    pub part: JavaType,
    pub name: String,
}

pub enum JavaType {
    Method,
    Class,
    Variable
}