extern crate tokens_lib;
use tokens_lib::*;
use std::{env, io::{BufReader, self, prelude::*, Error}, collections::HashMap};
use std::fs::File;

pub struct ItemSetData{
    pub transactions:Vec<String>,
    pub candidates:Vec<String>,
    pub itemSet:Vec<String>,
    pub finalFrequentItemSet:Vec<String>,
    pub frequentItems:HashMap<String,usize>,
    pub itemsCount:usize, countItemOccurrence:usize, displayFrequentItemSetNumber:usize, 
    pub minsup:usize,
    pub minconf:f32
}

fn main() -> io::Result<()> {

    // how can i error handle missing arguments here?
    let args: Vec<String> = env::args().collect(); //arg 1 is minsup and arg 2 is min confidence
    let fname = &args[1]; //args[0] is already reserved for something
    let file = File::open(fname)?;
    let reader = BufReader::new(file);
    let resp_transactions:Result<Vec<String>, Error> = reader.lines().collect::<Result<_,_>>();

    match resp_transactions {
        Ok(trans ) =>{
            let mut app_data = ItemSetData{
                transactions: trans,
                candidates:Vec::new(),
                itemSet:Vec::new(), // all known items
                finalFrequentItemSet:Vec::new(),
                frequentItems:HashMap::new(),
                itemsCount:0,
                countItemOccurrence:0,
                displayFrequentItemSetNumber:2,
                minsup:args[2].parse().expect("missing minimum support argument"),
                minconf:args[3].parse().expect("missing minimum confidence argument")
            };
        
            display(&mut app_data);
        },
        Err(why) =>{
            println!("Error: {:?}", why);
        }
    }
    
    Ok(())
}


fn display (data: &mut ItemSetData) -> Option<()>{
    for n in 0..data.transactions.len(){

        for word in data.transactions.get(n)?.to_string().split_whitespace(){
            if n == 0{
                data.itemSet.push(word.to_string())
            }else{
                if !data.itemSet.contains(&word.to_string()){
                    data.itemSet.push(word.to_string())
                }
            }
        }
    }
    data.itemsCount = data.itemSet.len();
    data.itemSet.sort();
    println!("Item set: {:?}", data.itemSet);
    println!("Number of items: {}", data.itemsCount);
    println!("Number of transactions: {}", data.transactions.len());

    first_frequent_item_set(data);
    Some(())
}

fn first_frequent_item_set(data:&mut ItemSetData){

    for item in &data.itemSet{
        data.countItemOccurrence = 0;

        for transaction_str in &data.transactions{

            if transaction_str.contains(item){
                data.countItemOccurrence+=1;
            }

        }

        if data.countItemOccurrence >= data.minsup{
            println!("{} => support = {}", item, data.countItemOccurrence);
            data.finalFrequentItemSet.push(item.to_string());
            data.frequentItems.insert(item.to_string(), data.countItemOccurrence);
        }
    }
    apriori_start(data);
}

fn apriori_start(data:&mut ItemSetData){
    let mut item_set_number = 1;

    for set in &data.finalFrequentItemSet{
        data.candidates.push(set.to_string());
    }

    while data.candidates.len() > 1{
        item_set_number += 1;
        generate_combinations(data, item_set_number);
        check_frequent_items(data);

        if data.candidates.len() <= 1{
            break;
        }
    }

    println!("\nAssociation Rules for Frequent Itemset");
    generate_association_rules(data);
}
    


fn generate_combinations(data: &mut ItemSetData, itr:i32) -> Option<()>{
    let mut candidates_temp:Vec<String> = Vec::new();
    let mut s1:String;
    let mut s2:String;
   

    if itr==2{
        for i in 0..data.candidates.len(){

            let mut str_tokenizer1 = FilteredTokenizer::new(filters::DefaultFilter{}, data.candidates.get(i)?);
            s1 = str_tokenizer1.next()?.term().to_string();
            
            for j in i+1..data.candidates.len(){
                let mut str_tokenizer2 = FilteredTokenizer::new(filters::DefaultFilter{}, data.candidates.get(j)?);
				s2 = str_tokenizer2.next()?.term().to_string();
                let add_string:String = String::from(s1.to_string() + " " + &s2.to_string());
                candidates_temp.push(add_string);
            }  
        }
    }else{
        for i in 0..data.candidates.len(){
            for j in i+1..data.candidates.len(){
                s1 = String::new();
                s2 = String::new();

                let mut str_tokenizer1 = FilteredTokenizer::new(filters::DefaultFilter{}, data.candidates.get(i)?);
                let mut str_tokenizer2 = FilteredTokenizer::new(filters::DefaultFilter{}, data.candidates.get(j)?);

                for _s in 0..itr-2{
                    s1 = s1 + " " + &str_tokenizer1.next()?.term().to_string();
                    s2 = s2 + " " + &str_tokenizer2.next()?.term().to_string();
                }

                if s2.eq_ignore_ascii_case(&s1){
                    let add_string:String = (s1 + " " + &str_tokenizer1.next()?.term().to_string() + " " + &str_tokenizer2.next()?.term().to_string()).trim().to_string();
                    candidates_temp.push(add_string);
                }
            }  
        }
    }
    data.candidates.clear();
    data.candidates = candidates_temp.clone(); 
    candidates_temp.clear();
    Some(())
}

fn check_frequent_items(data:&mut ItemSetData){

    let mut comb_list:Vec<String> = Vec::new();
    for candidate in &data.candidates{
        comb_list.push(candidate.to_string())
    } 
    println!("Frequent Itemset: {}", data.displayFrequentItemSetNumber);

    for str in comb_list{
        let mut flag = 0;
        let mut item_set_ocurrence:usize = 0;
        let words:Vec<String> = str.split_whitespace().map(|x| x.to_string()).collect::<Vec<String>>();
        let count = words.len();

        for transaction in &data.transactions{ 

            for word_str in &words{ 
                if transaction.contains(word_str){
                    flag+=1;
                }
            }
            if flag==count{
                item_set_ocurrence+=1;
            }
            flag=0;
        }

        if item_set_ocurrence >= data.minsup{
            println!("{} => support = {}", str, item_set_ocurrence);
            data.frequentItems.insert(str.clone(), item_set_ocurrence);
            data.finalFrequentItemSet.push(str.clone());
        }
        item_set_ocurrence = 0;
    }
    data.displayFrequentItemSetNumber += 1;

}

fn generate_association_rules(data:&mut ItemSetData) -> Option<()>{
    let mut confidence:f32;
    let mut confidence2:f32;

    for item in &data.finalFrequentItemSet{ 
        let value = data.frequentItems.get(item)?;
        let mut str:String = String::new();
        let mut str1:String = String::new();
        let words:Vec<String> = item.split_whitespace().map(|x| x.to_string()).collect::<Vec<String>>();
        let word_count_in_string = words.len();

        if word_count_in_string == 2{
            let s = data.frequentItems.get(&words[0])?;
            confidence = *value as f32/ *s as f32;

            if confidence>=data.minconf{
                println!("{} -> {} = Confidence = {} and Support = {}", words[0], words[1], confidence*100 as f32, value); 
            }
            let s1 = data.frequentItems.get(&words[1])?;
            confidence = *value as f32/ *s1 as f32;

            if confidence>=data.minconf{
                println!("{} -> {} = Confidence = {} and Support = {}", words[1], words[0], confidence*100 as f32, value);
            }
            
        }else{
            for i in 0..word_count_in_string-1{

                if i == 0{
                    str = str + &words[i];
                }else{
                    str = str + " " + &words[i];
                }

                for j in i+1..word_count_in_string{
                    str1 = str1 + " " + &words[j];
                }
                let s = data.frequentItems.get(&str)?;
                confidence = (*value as f32/ *s as f32);
                let st:String = str1.trim().to_string();
                let s1 = data.frequentItems.get(&st)?;
                confidence2 = (*value as f32/ *s1 as f32);

                if confidence >= data.minconf{
                println!("{} -> {} = Comfidemce = {} and Support = {}", str, str1, confidence*100 as f32, value);
                }

                if confidence2 >= data.minconf{
                println!("{} -> {} = Comfidemce = {} and Support = {}", st, str, confidence2*100 as f32, value);
                }
        
            str1="".to_string();   
            }
            str="".to_string();
            str1="".to_string();  
        }
    }
    Some(())
}  