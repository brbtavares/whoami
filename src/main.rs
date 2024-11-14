fn main() {
    println!("Hello, world!");
}

enum Cabelo_cor {
    preto,
    loiro,
    ruivo,
    castanho,
}

enum Pele_tonalidade {
    branca,
    parda,
    negra,
}

enum Sexo {
    masculino,
    feminino,
}

struct Personagem {
    cabelo_cor: Cabelo_cor,
    pele_tonalidade: Pele_tonalidade,
    sexo: Sexo,
    franja: bool,
}
