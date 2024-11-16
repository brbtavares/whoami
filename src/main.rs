use rand::prelude::*;
fn main() {
    let personagens: Vec<Personagem> = criar_personagens();
    let thread_rng = thread_rng();
    let mut rng: ThreadRng = thread_rng;
    let l = i32::try_from(personagens.len()).unwrap();
    let mut y = usize::try_from(rng.gen_range(0..l)).unwrap();
    println!("O personagens escolhido foi {}.", personagens[y].nome);
}

fn criar_personagens() -> Vec<Personagem> {
    let v = vec![
        Personagem {
            nome: String::from("SALLY"),
            cabelo_cor: CabeloCor::Castanho,
            pele_tonalidade: PeleTonalidade::Branca,
            sexo: Sexo::Feminino,
            franja: false,
        },
        Personagem {
            nome: String::from("MEG"),
            cabelo_cor: CabeloCor::Ruivo,
            pele_tonalidade: PeleTonalidade::Branca,
            sexo: Sexo::Feminino,
            franja: true,
        },
        Personagem {
            nome: String::from("KATE"),
            cabelo_cor: CabeloCor::Preto,
            pele_tonalidade: PeleTonalidade::Parda,
            sexo: Sexo::Feminino,
            franja: false,
        },
        Personagem {
            nome: String::from("JOSH"),
            cabelo_cor: CabeloCor::Loiro,
            pele_tonalidade: PeleTonalidade::Branca,
            sexo: Sexo::Masculino,
            franja: true,
        },
        Personagem {
            nome: String::from("KEVIN"),
            cabelo_cor: CabeloCor::Castanho,
            pele_tonalidade: PeleTonalidade::Branca,
            sexo: Sexo::Masculino,
            franja: false,
        },
        Personagem {
            nome: String::from("JULIA"),
            cabelo_cor: CabeloCor::Preto,
            pele_tonalidade: PeleTonalidade::Parda,
            sexo: Sexo::Feminino,
            franja: true,
        },
        Personagem {
            nome: String::from("LAURA"),
            cabelo_cor: CabeloCor::Loiro,
            pele_tonalidade: PeleTonalidade::Branca,
            sexo: Sexo::Feminino,
            franja: true,
        },
        Personagem {
            nome: String::from("JESS"),
            cabelo_cor: CabeloCor::Preto,
            pele_tonalidade: PeleTonalidade::Branca,
            sexo: Sexo::Feminino,
            franja: false,
        },
        Personagem {
            nome: String::from("PAT"),
            cabelo_cor: CabeloCor::Ruivo,
            pele_tonalidade: PeleTonalidade::Branca,
            sexo: Sexo::Feminino,
            franja: false,
        },
        Personagem {
            nome: String::from("ALEX"),
            cabelo_cor: CabeloCor::Loiro,
            pele_tonalidade: PeleTonalidade::Branca,
            sexo: Sexo::Feminino,
            franja: true,
        },
        Personagem {
            nome: String::from("SARA"),
            cabelo_cor: CabeloCor::Preto,
            pele_tonalidade: PeleTonalidade::Parda,
            sexo: Sexo::Feminino,
            franja: false,
        },
        Personagem {
            nome: String::from("MIKE"),
            cabelo_cor: CabeloCor::Preto,
            pele_tonalidade: PeleTonalidade::Negra,
            sexo: Sexo::Masculino,
            franja: false,
        },
    ];

    v
}

enum CabeloCor {
    Preto,
    Loiro,
    Ruivo,
    Castanho,
}

enum PeleTonalidade {
    Branca,
    Parda,
    Negra,
}

enum Sexo {
    Masculino,
    Feminino,
}

struct Personagem {
    nome: String,
    cabelo_cor: CabeloCor,
    pele_tonalidade: PeleTonalidade,
    sexo: Sexo,
    franja: bool,
}
