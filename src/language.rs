#![warn(dead_code)]

pub enum TokenType {
    ID,
    NUMBER,
    COMMA,
    COLON,
}

pub enum KeywordDef {
    Push,
    Sub,
    Mov,
}
pub struct Argumentnode<'a> {
    argument_code : i32,
    parent : &'a Keynode<'a>,
}
pub struct Keynode<'a> {
    keyword : KeywordDef,
    arguments : Vec<Argumentnode<'a>>,
}
pub struct Token {
    cont: String,
    tt: TokenType,
}

pub struct Lexer {
    input: String,
    position: usize,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn print_tokens(&self) {
        let mut i = 0;
        while i < self.tokens.len() {
            match self.tokens[i].tt {
                TokenType::ID => {
                    println!("[TOKEN]: ID");
                    i += 1;
                }
                TokenType::NUMBER => {
                    println!("[TOKEN]: NUMBER");
                    i += 1;
                }
                TokenType::COMMA => {
                    println!("[TOKEN]: COMMA");
                    i += 1;
                }
                TokenType::COLON => {
                    println!("[TOKEN]: COLON");
                    i += 1;
                }
            }
        }
    }

    pub fn new(input: String) -> Lexer {
        let mut lex = Lexer {
            input: input,
            position: 0,
            tokens: vec![],
        };
        lex.tokenize();
        lex
    }

    fn tokenize(&mut self) {
        while let Some(c) = self.input.chars().nth(self.position) {
            match c {
                ':' => {
                    self.tokens.push(Token {
                        cont: ":".to_string(),
                        tt: TokenType::COLON,
                    });
                    self.position += 1;
                }
                ',' => {
                    self.tokens.push(Token {
                        cont: ",".to_string(),
                        tt: TokenType::COMMA,
                    });
                    self.position += 1;
                }
                _ => {
                    let mut id = String::from(c);
                    if c.is_alphabetic() {
                        while let Some(next_c) = self.input.chars().nth(self.position + 1) {
                            if next_c.is_alphabetic() {
                                id.push(next_c);
                                self.position += 1;
                            } else {
                                break;
                            }
                        }
                        self.tokens.push(Token {
                            cont: id,
                            tt: TokenType::ID,
                        });
                    } else if c.is_numeric() {
                        while let Some(next_c) = self.input.chars().nth(self.position + 1) {
                            if next_c.is_numeric() {
                                id.push(next_c);
                                self.position += 1;
                            } else {
                                break;
                            }
                        }
                        self.tokens.push(Token {
                            cont: id,
                            tt: TokenType::NUMBER,
                        });
                    }
                    self.position += 1;
                }
            }
        }
    }
}
struct Tree<'a> {
    nodes: Vec<Keynode<'a>>
}
struct Parser<'a> {
    input : &'static str,
    lexer : Lexer,
    position : usize,
    tree : Tree<'a>
}

impl Parser<'_> {
    fn new(mut self, lexer : Lexer, input : &str){
        
    }
    fn make_tree(mut self) {
        let _push : String = "$push".to_string(); /*expect 1 arument */
        let _mov : String = "$mov".to_string(); /*expect 2 arguments */
        let _sub : String = "$sub".to_string(); /*auto gets the first two numbers in the stack */
        let _mul : String = "$mul".to_string();
        while self.position < self.lexer.tokens.len() {
            match self.lexer.tokens[self.position].tt {
                TokenType::COLON => {
                    todo!("TODO: implement colon for code blocks");
                }
                TokenType::COMMA => {
                    todo!("TODO: implement comma for arguments");
                }
                TokenType::ID => {
                    if self.lexer.tokens[self.position].cont.eq(&_push) {
                        self.tree.nodes.push(Keynode {
                            keyword: todo!(),
                            arguments: vec![],
                        });
                        self.position += 1;
                    } else if self.lexer.tokens[self.position].cont.eq(&_mov) {

                    } else if self.lexer.tokens[self.position].cont.eq(&_sub) {

                    } else if self.lexer.tokens[self.position].cont.eq(&_mul) {

                    }
                    self.position += 1;
                }
                TokenType::NUMBER => {
                    if let TokenType::ID = self.lexer.tokens[self.position - 1].tt {
                        self.tree.nodes[self.position - 1]
                        .arguments
                        .push(Argumentnode {
                            argument_code: self.lexer.tokens[self.position].cont.parse::<i32>().unwrap(),
                            parent: &self.tree.nodes[self.position - 1],
                        });
                    } else {
                        panic!("[ERROR]: No specified keyword for arguments");
                    }
                }
            }
            self.position += 1;
        }
    }
}