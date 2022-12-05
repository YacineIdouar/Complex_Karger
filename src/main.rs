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

fn plot_graph(i : usize, path : &str)->(){

    let mut file = match File::create(path){
        Ok(file) => file,
        Err(_) => panic!("Impossible d'ouvrir le fichier"),
    };   

    for i in 5..i{
        let mut matrice = vec![vec![0;i];i];// Matrice du début !
        let mut liste_arete : Vec<(i32,i32)> = Vec::new(); // Liste d'arete du début 

        // Phase d'initialisation
        matrice::init_matrice(&mut matrice, &mut liste_arete, i);
        

        let start = ProcessTime::try_now().expect("Process time failed");
        matrice::krager_matrice(&mut matrice,&mut liste_arete);
        let cpu_time: Duration = start.try_elapsed().expect("Process time failed");

        


        file.write_all(&(format!("{} \t {:?} \n",i,cpu_time.as_micros()))[..].as_bytes());
    }
}




fn main() {
    
    // Le main se concentre sur les appels aux fonctions de becnhmark 

    let nb_sommet = 100;

    plot_graph(nb_sommet, "matrice.txt");



    
}

/*Idée à devlopper => Ercire les différents graphe dans un fichier pour les print ! */