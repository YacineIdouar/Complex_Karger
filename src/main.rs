
use rand::Rng;


// Fonction permettant l'affichage des graphs !
fn dot_matrice (matrice : &Vec <Vec<i32>>) ->String {
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
    graph
}

// Fonction d'affichage des graphs représentés 

fn dot_liste (liste_adj : &mut Vec<Vec<i32>>)-> String{
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
    graph
}

// Initialisation de la matrice !!
fn init_matrice(matrice : &mut Vec <Vec<i32>>, liste_arete : &mut Vec<(i32,i32)>, n : usize) -> () {
    for i in 0..n {
        for j in (i+1)..n {
            if i==j {
                matrice[i][j] = 0;
            }else {
                let val = rand::thread_rng().gen_range(0..2);
                matrice[i][j] = val;
                matrice[j][i] = val;
                if val == 1 {
                    liste_arete.push((i.try_into().unwrap(),j.try_into().unwrap()));
                }
            }
        }
    }
}

// Fonction de contraction  sur la matrice d'adjacence 
fn contraction_matrice (matrice : &mut Vec <Vec<i32>>, liste_arete : &mut Vec<(i32,i32)>, sommet1 : usize, sommet2 : usize) -> (){
   // Retrait des arete de la liste des aretes 
        liste_arete.retain(|(x,y)| (!(*x==sommet2.try_into().unwrap() || *y == sommet2.try_into().unwrap())));
   
   // Mise a jour de la matrice
    for i in 0..matrice.len(){
        // Mise ajour de la ligne
        if i != sommet1{
        if matrice[sommet2][i] != -1 {
        matrice[sommet1][i]  += matrice[sommet2][i];
        matrice[i][sommet1] = matrice[sommet1][i];
        }

        for _j in 0..matrice[sommet2][i] {
            liste_arete.push((sommet1.try_into().unwrap(),i.try_into().unwrap()));
        }
    }
        // Supression du sommet en mettant toute ses valeurs a -1
        matrice[sommet2][i] = -1;
        matrice[i][sommet2] = -1;
    }
    
}

// Définiton d'une fonction size de la matrice d'adjacence

    fn taille (matrice : &Vec <Vec<i32>>)-> usize {
        let mut size = 0;
        for i in 0..matrice.len(){
            if matrice[i][i] == 0{
                size+=1;
            }
        }
        size
    }

    // Fonction de krager de la matrice  
    fn krager_matrice (matrice : &mut Vec <Vec<i32>>,liste_arete : &mut Vec<(i32,i32)>) -> (){
        // Tant que la taille de la matrice > 2 on réitère 
        while taille (&matrice) > 2  {
            let tirage = rand::thread_rng().gen_range(0..liste_arete.len());
            let (sommet1,sommet2) = liste_arete[tirage];
            contraction_matrice(matrice,liste_arete, sommet1.try_into().unwrap(), sommet2.try_into().unwrap());  
        }
    }

    // On passe la fonction de krager avec un meilleur résultat 

    fn karger_iter_matrice (matrice : Vec <Vec<i32>>,liste_arete : Vec<(i32,i32)>)-> (Vec <Vec<i32>>,usize){
        let n = matrice.len()^2;
        let mut min = liste_arete.len();
        let mut mat_res = matrice.clone();
        for _ in 0..n{
            let mut matrice2 = matrice.clone();
            let mut liste_arete2  = liste_arete.clone();
            krager_matrice(&mut matrice2,&mut liste_arete2);
            if liste_arete2.len() < min{
                min = liste_arete2.len();
                mat_res = matrice2.clone();
            }
        }
        (mat_res,min)
    }


    /* Réalisationde la deuxième implémentation de la matrice avec des listes d'adjacence ! */

    // Initialisation de la liste d'adjacence  
    fn initListeAdj(liste_adj : &mut Vec<Vec<i32>>) -> (){

        // On parcours tous les noeuds !
        for i in 0..liste_adj.len(){
            // On choisit un nombre d'aretes aléatoire !
            let nb_arete:i32 =rand::thread_rng().gen_range(1..liste_adj.len().try_into().unwrap()) - (liste_adj[i].len() as i32);

            for _ in 0..nb_arete  {
                let mut sommet_arrive = rand::thread_rng().gen_range(0..liste_adj.len().try_into().unwrap()); // On choisit le second sommet
                // On vérifie que le sommet ne pointe pas sur lui meme et que le sommet sélectionné n'est pas déja dans la liste
                let mut iter = liste_adj[i].iter();
                while sommet_arrive == i || (iter.find (|&&x| x == sommet_arrive.try_into().unwrap()) != None){
                       
                        iter = liste_adj[i].iter();
                        sommet_arrive = rand::thread_rng().gen_range(0..liste_adj.len());
                }   
                liste_adj[i].push(sommet_arrive.try_into().unwrap());   
                liste_adj[sommet_arrive].push(i.try_into().unwrap());
            }
        }
    }



    // Contraction des listes d'adjacences !
    fn contraction_liste (liste_adj : &mut Vec<Vec<i32>>, sommet1 : usize, sommet2 : usize) -> (){

        // Nettoyage de la première liste O(n)
        let mut i : usize = 0;
        while i < liste_adj[sommet1].len(){
            if liste_adj[sommet1][i] == sommet2.try_into().unwrap(){
                liste_adj[sommet1].remove(i);
            }else{
                i+=1;
            }
        }

        // Fusion des deux listes ! O(n)
        for i in 0..liste_adj[sommet2].len() {

            let val = liste_adj[sommet2][i];

            if   val!= sommet1.try_into().unwrap(){
                liste_adj[sommet1].push(val);
                for j in 0.. liste_adj[val as usize].len() {
                    if liste_adj[val as usize][j] == sommet2.try_into().unwrap(){
                        liste_adj[val as usize][j] = sommet1.try_into().unwrap();
                    }
                }
            }
            // On supprime la liste en mettant un valeur différente 
        }
        
        // On met à jour les indices de la liste ! O(n)
        for i in 0..liste_adj.len(){
            for j in 0..liste_adj[i].len()  {
                    if liste_adj[i][j] > sommet2.try_into().unwrap(){
                        liste_adj[i][j] -=1; // On décrémente tous les indices > au sommet supprimé !
                    }
            }   
        }
       
        liste_adj.remove(sommet2);

    }

    fn krager_liste_adj (liste_adj : &mut Vec<Vec<i32>>)->(){     
            
        while liste_adj.len() > 2{
            // Choix des deux sommet à contrater !
            let sommet1 = rand::thread_rng().gen_range(0..liste_adj.len());

            
            let indice_sommet2 =  rand::thread_rng().gen_range(0..liste_adj[sommet1].len()) as usize;
            let sommet2 = liste_adj[sommet1][indice_sommet2] as usize;
            

            // Appel de la fonction de contraction 

            contraction_liste(liste_adj, sommet1, sommet2);
        }

    }

// Implémentation de l'algorithme de karger_Sein 

fn krager_matrice_partiel (matrice : &mut Vec <Vec<i32>>,liste_arete : &mut Vec<(i32,i32)>,nb_con : usize) -> (){
    // Tant que la taille de la matrice > 2 on réitère 
    while taille (&matrice) > nb_con {
        let tirage = rand::thread_rng().gen_range(0..liste_arete.len());
        let (sommet1,sommet2) = liste_arete[tirage];
        contraction_matrice(matrice,liste_arete, sommet1.try_into().unwrap(), sommet2.try_into().unwrap());  
    }
}

// ALgorithme de Karger stein

    fn  karger_Stein(matrice : &mut Vec <Vec<i32>>,liste_arete : &mut Vec<(i32,i32)>)-> (Vec <Vec<i32>>,usize){
        if taille(&matrice) <= 6 {
            krager_matrice(matrice, liste_arete);
            return (*matrice,liste_arete.len());
        }
        
          let number_contration = (1 as f32 + taille(&matrice) as f32/1.41) as usize + 1;
          // Matrice1 pour le premier calcul
          let mut matrice1 = matrice.clone();
          let mut liste_arete1 = liste_arete.clone();
          // Matrice2 pour le second calcul
          let mut matrice2 = matrice.clone();
          let mut liste_arete2 = liste_arete.clone();
          // Realisation de la contraction partiel
          krager_matrice_partiel(&mut matrice1, &mut liste_arete1, number_contration);
          krager_matrice_partiel(&mut matrice2, &mut liste_arete2, number_contration);
          // Rappel de la fonction sur les matrice contracté
          let (matrice_res1,taille1) = karger_Stein(&mut matrice1, &mut liste_arete1);
          let (matrce_res2,taille2) = karger_Stein(&mut matrice2, &mut liste_arete2);

          // On retourne la matrice avec la plus petite taille !
          if taille1 < taille2 {
            return (matrce_res2,taille2);
          }

          (matrice_res1,taille1)     

    }





fn main() {
    
    let n : usize = 10;
    
    // Gestionde la matrice !
    let mut matrice = vec![vec![0;n];n];// Matrice du début !
    let mut liste_arete : Vec<(i32,i32)> = Vec::new(); // Liste d'arete du début 

    init_matrice(&mut matrice, &mut liste_arete, n);

    let mut matrice_res = vec![vec![0;n];n];
    let mut min = 0;

    for i in 0..n{
        println!("{:?}",matrice[i]);
    }

    println!("Le graphe dot avant : \n {}",dot_matrice(&matrice));

    println!("Sorie de l'algorithme !!");

    (matrice_res,min) = karger_Stein(&mut matrice, &mut liste_arete);
    

    println!("Le graphe dot après : \n {}",dot_matrice(&matrice_res));


    // Gestion de la liste d'adjcence !
   /*  let mut liste_adj : Vec<Vec<i32>> = vec![vec![];n];

    initListeAdj(&mut liste_adj);

    println!("{}",dot_liste(&mut liste_adj));

    krager_liste_adj(&mut liste_adj);
    

    println!("{}",dot_liste(&mut liste_adj));*/


    
}

/*Idée à devlopper => Ercire les différents graphe dans un fichier pour les print ! */