use rand::prelude::*;
fn main() {
    let mut jogo = Jogo::new();
    jogo.jogar_dados();
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

struct Jogador {
    nome: String,
    personagem: Option<Personagem>,
    universo_de_busca: Option<Vec<Personagem>>,
}

struct Jogo {
    rodadas: Vec<Rodada>,
    rng: ThreadRng,
    jogadores: Vec<Jogador>,
    personagens: Vec<Personagem>,
}

struct Rodada {
    idx: i32,
    idxJogadorVencedor: i32,
    jogadas: i32,
}

impl Jogo {
    pub fn new() -> Self {
        Self {
            rodadas: Vec::new(),
            rng: thread_rng(),
            personagens: Self::criar_personagens(),
            jogadores: Self::criar_jogadores(),
        }
    }

    pub fn jogar_dados(&mut self) {
        let i = usize::try_from(
            self.rng
                .gen_range(0..i32::try_from(self.personagens.len()).unwrap()),
        )
        .unwrap();
        println!("O personagem escolhido foi {}.", self.personagens[i].nome);
    }

    fn criar_jogadores() -> Vec<Jogador> {
        let v = vec![
            Jogador {
                nome: String::from("ALICE"),
                personagem: None,
                universo_de_busca: None,
            },
            Jogador {
                nome: String::from("BOB"),
                personagem: None,
                universo_de_busca: None,
            },
        ];
        v
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
}
