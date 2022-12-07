//Appel des bibliothèques externes
use std::fs::File;
use std::io::Write;
use std::time::Duration;
use cpu_time::ProcessTime;

// Appel des modules
mod matrice;
mod liste_adj;
mod dot;

fn plot_graph(nb_sommet : usize, path : &str)->(){

    let mut file = match File::create(path){
        Ok(file) => file,
        Err(_) => panic!("Impossible d'ouvrir le fichier"),
    };   

    for i in 5..nb_sommet{
        let mut matrice = vec![vec![0;i];i];// Matrice du début !
        let mut liste_arete : Vec<(i32,i32)> = Vec::new(); // Liste d'arete du début 
        // Initialisation de la matrice 
        matrice::init_matrice(&mut matrice, &mut liste_arete, i);


        let mut liste_adj : Vec<Vec<i32>> = vec![vec![];i];
        let mut nombre_arete = liste_adj::initListeAdj(&mut liste_adj); // Initialisation de la liste d'adjacence


        // Début du benchmark
        let start = ProcessTime::try_now().expect("Process time failed");
        matrice::krager_matrice(&mut matrice,&mut liste_arete); // Appel de la fonction de krager sur la matrice
        let cpu_time: Duration = start.try_elapsed().expect("Process time failed");

        let start2 = ProcessTime::try_now().expect("Process time failed");
        liste_adj::krager_liste_adj(&mut liste_adj, &mut nombre_arete); // Appel de la fonction sur la liste
        let cpu_time2: Duration = start2.try_elapsed().expect("Process time failed");

        file.write_all(&(format!("{} \t {:?} \t {:?} \n",i,cpu_time.as_micros(),cpu_time2.as_micros()))[..].as_bytes()).expect("Erreur d'écriture");
    }


}




fn main() {
    
    // Le main se concentre sur les appels aux fonctions de becnhmark 

    let nb_sommet = 150;
    plot_graph(nb_sommet, "comparaison_matrice_liste.txt");

    /*let mut liste_adj : Vec<Vec<i32>> = vec![vec![];nb_sommet];
     let mut nombre_arete = liste_adj::initListeAdj(&mut liste_adj); // Initialisation de la liste d'adjacence
     liste_adj::krager_liste_adj(&mut liste_adj, &mut nombre_arete); // Appel de la fonction sur la liste
    dot::dot_liste(&mut liste_adj);
    //println!("NOmbre d'arete : {}",nombre_arete);*/
}

/*Idée à devlopper => Ercire les différents graphe dans un fichier pour les print ! */