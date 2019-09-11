use std::fmt;
use std::fs::File;
use std::io::prelude::*;

// Traits funcionam como interfaces em Java, mas possuem algumas funcionalidades a mais
trait Leitor {
    // Implementação de função sem definição
    fn new(nome: &str) -> Self;

    fn conteudo(&self) -> &String;

    // Implementação com definição padrão
    // É possivel dar override
    fn reverter_conteudo(&self) -> String {
        let mut v: Vec<&str> = self.conteudo().split_whitespace().collect();
        v.reverse();

        let mut s = String::new();

        for p in v.iter() {
            s.push_str(p);
        }

        s
    }
}

// O macro 'Derive' serve para implementar Traits sem escrever muito código, desde que todos os tipos da
// 'struct' também implememtem esse trait
#[derive(Debug, Clone)]
struct Arquivo {
    nome: String,
    inner: String,
}

// 'impl' criam os métodos da struct assim como um DAO no Java
impl Arquivo {
    fn duplicar(&self) {
        let mut nf = File::create(&format!("{}.duplicado", self.nome)).unwrap();

        nf.write_all(self.inner.as_bytes()).unwrap();
    }
}

// É possível escrever implementações personalizadas dos Traits da propria linguagem
impl fmt::Display for Arquivo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

// Não implementar qualquer um destas funções causa um erro do compilador por a Trait não estar
// sendo implementada completamente
impl Leitor for Arquivo {
    fn new(nome: &str) -> Arquivo {
        let mut f = File::open(nome).unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();

        Arquivo {
            nome: nome.to_string(),
            inner: s,
        }
    }

    fn conteudo(&self) -> &String {
        &self.inner
    }
}

#[derive(Debug, Clone)]
struct Bytes<T> {
    // Exemplo de tipos genéricos. Esta 'struct' pode ser criada com qualquer outra srtuct 'T'
    source: T, // desde que o tipo 'T' implemente os Traits citados do 'derive'
    byte_count: i64,
}

// As funções abaixo só podem ser acessadas por Bytes se o tipo 'T' implementar Leitor e Clone
impl<T: Leitor + Clone> Bytes<T> {
    fn contar_bytes(x: &T) -> Bytes<T> {
        let mut c: i64 = 0;

        let biter = x.conteudo().as_bytes();

        for b in biter {
            c += *b as i64;
        }

        Bytes {
            source: x.clone(),
            byte_count: c,
        }
    }
}

fn main() {
    let x: Arquivo = Leitor::new("skylab.txt");
    let y = Bytes::contar_bytes(&x);

    println!("Conteudo e bytes: {:#?}", y);
    println!("Conteudo reverso: {}", x.reverter_conteudo());

    x.duplicar();
}
