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
        // Faire les moyennes sur les sommets.
        /*let mut ecrire = 0;

        for _ in 0..10{
        let mut matrice = vec![vec![0;i];i];// Matrice du début !
        let mut liste_arete : Vec<(i32,i32)> = Vec::new(); // Liste d'arete du début 
        // Initialisation de la matrice 
        matrice::init_matrice(&mut matrice, &mut liste_arete, i);

        if liste_arete.len() == 1{
                ecrire += 1;
        }
    }*/ 
        
        
        // Début des benchmarks 

        let mut matrice2 = vec![vec![0;i];i];// Matrice du début !
        let mut liste_arete2 : Vec<(i32,i32)> = Vec::new(); // Liste d'arete du début 
        matrice::init_matrice(&mut matrice2, &mut liste_arete2, i);

        let mut liste_adj : Vec<Vec<i32>> = vec![vec![];nb_sommet];
        liste_adj::initListeAdj(&mut liste_adj);
    

        // Début du benchmark
        let start = ProcessTime::try_now().expect("Process time failed");
        liste_adj::initListeAdj(&mut liste_adj) ;// Appel de la fonction de krager sur la matrice
        let cpu_time: Duration = start.try_elapsed().expect("Process time failed");

        let start2 = ProcessTime::try_now().expect("Process time failed");
        matrice::krager_matrice(&mut matrice2, &mut liste_arete2);// Appel de la fonction sur la liste
        let cpu_time2: Duration = start2.try_elapsed().expect("Process time failed");
        
     
        // Ecriture dans le fichier !
        file.write_all(&(format!("{} \n",i))[..].as_bytes()).expect("Erreur d'écriture");
        
    }


}




fn main() {
    
    // Le main se concentre sur les appels aux fonctions de becnhmark 

    //Faire des benchmarke

    let nb_sommet = 150;
    plot_graph(nb_sommet, "Reussite karger");

    
    // Utiliser Pour tester les résultats 

    /*let mut matrice = vec![vec![0;nb_sommet];nb_sommet];// Matrice du début !
    let mut liste_arete : Vec<(i32,i32)> = Vec::new(); // Liste d'arete du début 
    // Initialisation de la matrice 
    matrice::init_matrice(&mut matrice, &mut liste_arete, nb_sommet);


    let mut liste_adj : Vec<Vec<i32>> = vec![vec![];nb_sommet];
    liste_adj::initListeAdj(&mut liste_adj);

   //(matrice_res,taille) =  matrice::karger_Stein(&mut matrice, &mut liste_arete);
    //dot::dot_matrice(&matrice);
    dot::dot_liste(& mut liste_adj);*/   
}

/*Idée à devlopper => Ercire les différents graphe dans un fichier pour les print ! */