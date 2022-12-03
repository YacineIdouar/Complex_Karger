//Appel des bibliothèques externes
use rand::Rng;
use plotters::prelude::*;
use std::fs::File;
use std::io::Write;
use std::time::Duration;
use cpu_time::ProcessTime;

// Appel des modules
mod matrice;
mod liste_adj;
mod dot;

fn plot_graph(i : usize)->(){

    let mut file = match File::create("benchmark.txt"){
        Ok(file) => file,
        Err(_) => panic!("Impossible d'ouvrir le fichier"),
    };   
    let ligne = String::new();
    for i in 5..i{
        let mut matrice = vec![vec![0;i];i];// Matrice du début !
        let mut liste_arete : Vec<(i32,i32)> = Vec::new(); // Liste d'arete du début 

        matrice::init_matrice(&mut matrice, &mut liste_arete, i);
        let start = ProcessTime::try_now().expect("Process time failed");
        matrice::krager_matrice(&mut matrice, &mut liste_arete);
        let cpu_time: Duration = start.try_elapsed().expect("Process time failed");
        file.write_all(&(format!("{} \t {:?}\n",i,cpu_time.as_micros()))[..].as_bytes());
    }
}




fn main() {
    
    let n : usize = 30;
    
    // Gestionde la matrice !
   /* let mut matrice = vec![vec![0;n];n];// Matrice du début !
    let mut liste_arete : Vec<(i32,i32)> = Vec::new(); // Liste d'arete du début 

    matrice::init_matrice(&mut matrice, &mut liste_arete, n);

    let mut matrice2 = matrice.clone();
    let mut liste_arete2 = liste_arete.clone();

    let mut matrice_res = vec![vec![0;n];n];
    let mut min = 0;

    for i in 0..n{
        println!("{:?}",matrice[i]);
    }

    println!("Le graphe dot avant : \n {}",dot_matrice(&matrice));

    println!("Sorie de l'algorithme !!");

    (matrice_res,min) = matrice::karger_Stein(&mut matrice, &mut liste_arete);
    

    println!("Le graphe dot après : \n {}",dot_matrice(&matrice_res));

    println!("Sortie de l'algorithme iteratif ! ");
    let mut matrice_res2 = vec![vec![0;n];n];
    let mut min2 = 0;

    (matrice_res2,min2) = matrice::karger_iter_matrice(matrice2, liste_arete2);

    println!("Le graphe dot après : \n {}",dot_matrice(&matrice_res2));*/


    // Gestion de la liste d'adjcence !
    let mut liste_adj : Vec<Vec<i32>> = vec![vec![];n];

   /*  liste_adj::initListeAdj(&mut liste_adj);
    dot::dot_liste(&mut liste_adj);
    liste_adj::krager_liste_adj(&mut liste_adj);
    dot::dot_liste(&mut liste_adj);*/
    plot_graph(150);



    
}

/*Idée à devlopper => Ercire les différents graphe dans un fichier pour les print ! */