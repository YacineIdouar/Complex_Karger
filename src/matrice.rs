use rand::Rng;

// Fonction d'initialisation de la matrice d'adjacence.
pub fn init_matrice(matrice : &mut Vec <Vec<i32>>, liste_arete : &mut Vec<(i32,i32)>, n : usize) -> () {
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


// Fonction de contraction  sur la matrice d'adjacence, Définit en mode private car à usage algorithmique.
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


 // Définiton d'une fonction size de la matrice d'adjacence pour s'adapter au choix d'implémentation
 pub fn taille (matrice : &Vec <Vec<i32>>)-> usize {
    let mut size = 0;
    for i in 0..matrice.len(){
        if matrice[i][i] == 0{
            size+=1;
        }
    }
    size
}


// Fonction de krager de la matrice  
pub fn krager_matrice (matrice : &mut Vec <Vec<i32>>,liste_arete : &mut Vec<(i32,i32)>) -> (){
    // Tant que la taille de la matrice est supérieure à 2 on réitère 
    while taille (&matrice) > 2  {
        let tirage = rand::thread_rng().gen_range(0..liste_arete.len());
        let (sommet1,sommet2) = liste_arete[tirage];
        contraction_matrice(matrice,liste_arete, sommet1.try_into().unwrap(), sommet2.try_into().unwrap());  
    }
}

 // On Améliore le résultat de la fonction de karger en itérant n² sur le résultat 
 pub fn karger_iter_matrice (matrice : Vec <Vec<i32>>,liste_arete : Vec<(i32,i32)>)-> (Vec <Vec<i32>>,usize){
    let nombre_iter = matrice.len()^2;
    
    // Déclaration des valeurs de retour 
    let mut min = liste_arete.len(); // Initialisation au nombre d'arête de la matrice
    let mut mat_res = matrice.clone(); // Initialisation à la matrice de départ
    
    for _ in 0..nombre_iter{
        let mut matrice2 = matrice.clone(); // Copie de la matrice 
        let mut liste_arete2  = liste_arete.clone(); // Copie de la liste d'arete
        krager_matrice(&mut matrice2,&mut liste_arete2); // Appel à la fonction de krager
        
        // On sauvgarde la coupe minimale
        if liste_arete2.len() < min{
            min = liste_arete2.len();
            mat_res = matrice2.clone();
        }
    }
    (mat_res,min)
}

// Définition de krager partiel à usage algorithmique pour Krager-Stein
fn krager_matrice_partiel (matrice : &mut Vec <Vec<i32>>,liste_arete : &mut Vec<(i32,i32)>,nb_con : usize) -> (){
    // Tant que la taille de la matrice est supérieur aux nombre de contraction on réitère 
    while taille (&matrice) > nb_con {
        let tirage = rand::thread_rng().gen_range(0..liste_arete.len());
        let (sommet1,sommet2) = liste_arete[tirage];
        contraction_matrice(matrice,liste_arete, sommet1.try_into().unwrap(), sommet2.try_into().unwrap());  
    }
}

// Implémentation de l'algorithme de Krager-Stein
pub fn  karger_Stein(matrice : &mut Vec <Vec<i32>>,liste_arete : &mut Vec<(i32,i32)>)-> (Vec <Vec<i32>>,usize){
    if taille(&matrice) <= 6 {
        let mut matrice_fin = matrice.clone();
        let mut liste_arete_fin = liste_arete.clone();
        krager_matrice(&mut matrice_fin, &mut liste_arete_fin);
        return (matrice_fin,liste_arete_fin.len());
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
      let (matrice_res2,taille2) = karger_Stein(&mut matrice2, &mut liste_arete2);

      // On retourne la matrice avec la plus petite taille !
      if taille1 < taille2 {
        return (matrice_res1,taille1);
      }

      (matrice_res2,taille2)     

}