fn main() {
    let args: Vec<String> = std::env::args().collect();
    for arg in &args {
        println!("{arg}");
        println!("{:?}", args);
    }
    // Q1 : Oui, c'est aussi String
    // Q2 : Grâce à []. Avec [T,N], on restreint la taille lors de la création du tableau.
    // Q3 : L'inférence de type?
    // Q4 : args.len(). C'est possible en C (args[1])
    // Q5 : println! affiche le message dans le shell. {} permet d'insérer des variables sans avoir
    // à faire des précisions sur les types.

    //Q6 On n'a pas à créer une variable i pour itérer. Les écritures avec &args et args.iter sont
    //simillaire. Ne pas mettre l'& fait que le tableau args se fait consommer par l'itération et
    // ne peux donc pluus être récupéré par le println!("{:?}", args);
    // Le .collect sert à récupérer la liste des arguments. Sans lui, on ne peux l'extraire.
}