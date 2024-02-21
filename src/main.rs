mod lexical_parser;

fn main() {
    let mut teste = lexical_parser::Automaton::new();

    let a = teste.process_input("program teste; {programa exemplo}\nvar\n\tvalor1: integer;\n\tvalor2: float;\nbegin\n\tvalor1 :=10;\nend.".to_string());

    for lexeme in a {
        println!("{:?}", lexeme)
    }
}
