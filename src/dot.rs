use std::fs::File;
use std::io::Write;

// Fonction permettant l'affichage des graphs !
pub fn dot_matrice (matrice : &Vec <Vec<i32>>) ->() {
    let mut file = match File::create("print.gv"){
        Ok(file) => file,
        Err(_) => panic!("Impossible d'ouvrir le fichier"),
    };   
    let mut graph = String::new();
    graph = format! ("{}{}",graph,String::from("graph {"));
    for i in 0..matrice.len() {
        for j in (i+1)..matrice.len(){
            for _ in 0..matrice[i][j]{
               graph = format!("{}{}{}{}{}",graph ,i.to_string(),String::from(" -- "), j.to_string(),String::from(" \n"));
            }
        }
    }
    graph = format! ("{}{}",graph,String::from("}"));
    let to_write = &graph[..];

    file.write_all(to_write.as_bytes()).expect("Erreur exriture");
}

// Fonction d'affichage des graphs représentés 
pub fn dot_liste (liste_adj : &mut Vec<Vec<i32>>)-> (){

    let mut file = match File::create("print.gv"){
        Ok(file) => file,
        Err(_) => panic!("Impossible d'ouvrir le fichier"),
    };  
    
    let mut graph = String::new();
    graph = format! ("{}{}",graph,String::from("graph {"));
    for i in 0..liste_adj.len() {
        for j in 0..liste_adj[i].len(){
                if liste_adj[i][j] != -1 && (liste_adj[i][j]) as usize > i{
               graph = format!("{}{}{}{}{}",graph ,i.to_string(),String::from(" -- "), liste_adj[i][j].to_string(),String::from(" \n"));
                }
        }
    }
    graph = format! ("{}{}",graph,String::from("}"));
    let to_write = &graph[..];

    file.write_all(to_write.as_bytes()).expect("Erreur exriture");
}