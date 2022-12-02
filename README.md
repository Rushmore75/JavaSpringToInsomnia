# JavaSpringToInsomnia
### How it works:
After reverse engineering an exported workspace from [Insomnia](https://insomnia.rest/) I rebuilt it into Rust with the help of `serde_json`. The code then scapes thru the givin file looking for Java annotations, specifically `@RequestMapping` from [Spring](https://spring.io/) as it's configured right now.

It then grabs the data concerning what kind of request it should be and what the url is.

It also guess at what it should be named based on the attached method name.

Then it bundles this all together back into the json format, ready to be imported into Insomnia again.

---
### To do:
- [ ] Add ability to be run as a command line utility.