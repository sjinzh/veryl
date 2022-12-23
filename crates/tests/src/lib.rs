#[cfg(test)]
mod parser {
    use std::fs;
    use veryl_parser::Parser;

    fn test(name: &str) {
        let file = format!("../../testcases/vl/{}.vl", name);
        let input = fs::read_to_string(&file).unwrap();
        let ret = Parser::parse(&input, &file);
        match ret {
            Ok(_) => assert!(true),
            Err(err) => println!("{}", err),
        }
    }

    include!(concat!(env!("OUT_DIR"), "/test.rs"));
}

#[cfg(test)]
mod analyzer {
    use std::fs;
    use veryl_analyzer::Analyzer;
    use veryl_parser::Parser;

    fn test(name: &str) {
        let file = format!("../../testcases/vl/{}.vl", name);
        let input = fs::read_to_string(&file).unwrap();

        let ret = Parser::parse(&input, &file).unwrap();
        let mut analyzer = Analyzer::new(&input);
        analyzer.analyze(&ret.veryl);

        assert!(analyzer.errors.is_empty());
    }

    include!(concat!(env!("OUT_DIR"), "/test.rs"));
}

#[cfg(test)]
mod formatter {
    use std::fs;
    use veryl_config::Config;
    use veryl_formatter::Formatter;
    use veryl_parser::Parser;

    fn test(name: &str) {
        let config_path = Config::search_from_current().unwrap();
        let config = Config::load(&config_path).unwrap();

        let file = format!("../../testcases/vl/{}.vl", name);
        let input = fs::read_to_string(&file).unwrap();
        let original = input.clone();

        // minify without lines which contain line comment
        let mut minified = String::new();
        for line in input.lines() {
            if line.contains("//") {
                minified.push_str(&format!("{}\n", line));
            } else {
                minified.push_str(&format!("{}\n", line.replace(' ', "")));
            }
        }

        let ret = Parser::parse(&input, &file).unwrap();
        let mut formatter = Formatter::new(&config);
        formatter.format(&ret.veryl);

        // remove CR on Windows environment
        let original = original.replace('\r', "");

        assert_eq!(original, formatter.as_str());
    }

    include!(concat!(env!("OUT_DIR"), "/test.rs"));
}

#[cfg(test)]
mod emitter {
    use std::fs;
    use veryl_config::Config;
    use veryl_emitter::Emitter;
    use veryl_parser::Parser;

    fn test(name: &str) {
        let config_path = Config::search_from_current().unwrap();
        let config = Config::load(&config_path).unwrap();

        let file = format!("../../testcases/vl/{}.vl", name);
        let input = fs::read_to_string(&file).unwrap();

        let ret = Parser::parse(&input, &file).unwrap();
        let mut emitter = Emitter::new(&config);
        emitter.emit(&ret.veryl);

        let file = format!("../../testcases/sv/{}.sv", name);
        let reference = fs::read_to_string(&file).unwrap();

        // remove CR on Windows environment
        let reference = reference.replace('\r', "");

        assert_eq!(reference, emitter.as_str());
    }

    include!(concat!(env!("OUT_DIR"), "/test.rs"));
}
