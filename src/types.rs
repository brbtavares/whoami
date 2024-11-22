use rand::prelude::*;

impl Jogo {
    pub fn new() -> Self {
        Self {
            rodadas: Vec::new(),
            rng: thread_rng(),
            universo_de_busca: Self::criar_universo_de_busca(),
            personagem: None,
        }
    }

    pub fn atribuir_personagem(&mut self) {
        let random_number: usize = self.rng.gen_range(0..12) as usize;
        self.personagem = Some(self.universo_de_busca[random_number].clone());
    }

    fn criar_universo_de_busca() -> Vec<Personagem> {
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
}

#[derive(Clone)]
struct Personagem {
    nome: String,
    cabelo_cor: CabeloCor,
    pele_tonalidade: PeleTonalidade,
    sexo: Sexo,
    franja: bool,
}

#[derive(Clone)]
pub struct Jogo {
    rodadas: Vec<Rodada>,
    rng: ThreadRng,
    personagem: Option<Personagem>,
    universo_de_busca: Vec<Personagem>,
}

#[derive(Clone)]
struct Rodada {
    idx: i32,
}

#[derive(Clone)]
enum CabeloCor {
    Preto,
    Loiro,
    Ruivo,
    Castanho,
}

#[derive(Clone)]
enum PeleTonalidade {
    Branca,
    Parda,
    Negra,
}

#[derive(Clone)]
enum Sexo {
    Masculino,
    Feminino,
}
