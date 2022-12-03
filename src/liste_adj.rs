use rand::Rng;

// Initialisation de la liste d'adjacence  
pub fn initListeAdj(liste_adj : &mut Vec<Vec<i32>>) -> (){

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
    liste_adj.remove(sommet2);// SUppression du sommet de liste en O(n)
}


pub fn krager_liste_adj (liste_adj : &mut Vec<Vec<i32>>)->(){     
            
    while liste_adj.len() > 2{
        // Choix des deux sommet à contrater !
        let sommet1 = rand::thread_rng().gen_range(0..liste_adj.len());

        
        let indice_sommet2 =  rand::thread_rng().gen_range(0..liste_adj[sommet1].len()) as usize;
        let sommet2 = liste_adj[sommet1][indice_sommet2] as usize;
        

        // Appel de la fonction de contraction 
        contraction_liste(liste_adj, sommet1, sommet2);
    }

}