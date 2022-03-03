extern crate tokens_lib;
use tokens_lib::*;
use std::{env, io::{BufReader, self, prelude::*}, slice::SliceIndex, collections::HashMap, ops::Add};
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

    let mut appData = ItemSetData{
        transactions: reader.lines().collect::<Result<_,_>>().unwrap(),
        candidates:Vec::new(),
        itemSet:Vec::new(), // all known items
        finalFrequentItemSet:Vec::new(),
        frequentItems:HashMap::new(),
        itemsCount:0,
        countItemOccurrence:0,
        displayFrequentItemSetNumber:2,
        minsup:args[2].parse().expect("missing min supp argument"),
        minconf:args[3].parse().expect("missing min confidence argument")
    };

    /* let mut transactions: Vec<String> = Vec::new();
    for line in reader.lines(){
        transactions.push(line.unwrap());
    } */

    display(&mut appData);
    Ok(())
}


fn display (data: &mut ItemSetData){
    for n in 0..data.transactions.len(){

        for word in data.transactions.get(n).unwrap().to_string().split_whitespace(){
            if n == 0{
                data.itemSet.push(word.to_string())
            }else{
                if !data.itemSet.contains(&word.to_string()){
                    data.itemSet.push(word.to_string())
                }
            }
            
        }
        /* let string: String = data.transactions.get(n).unwrap().to_string();
        let words:Vec<String> = string.split_whitespace().map(|x| x.to_string()).collect::<Vec<String>>();*/
    }
    data.itemsCount = data.itemSet.len();
    data.itemSet.sort();
    println!("Item set: {:?}", data.itemSet);
    println!("Number of items: {}", data.itemsCount);
    println!("Number of transactions: {}", data.transactions.len());

    firstFrequentItemSet(data);
}

fn firstFrequentItemSet(data:&mut ItemSetData){

    for item in &data.itemSet{
        data.countItemOccurrence = 0;

        for transactionStr in &data.transactions{

            if transactionStr.contains(item){
                data.countItemOccurrence+=1;
            }

        }

        if data.countItemOccurrence >= data.minsup{
            println!("{} => support = {}", item, data.countItemOccurrence);
            data.finalFrequentItemSet.push(item.to_string());
            data.frequentItems.insert(item.to_string(), data.countItemOccurrence);
        }
    }
    aprioriStart(data);
}

fn aprioriStart(data:&mut ItemSetData){
    let mut itemSetNumber = 1;

    for set in &data.finalFrequentItemSet{
        data.candidates.push(set.to_string());
    }

    while data.candidates.len() > 1{
        itemSetNumber += 1;
        generateCombinations(data, itemSetNumber);
        checkFrequentItems(data);

        if data.candidates.len() <= 1{
            break;
        }
    }

    println!("\nAssociation Rules for Frequent Itemset");
    generateAssociationRules(data);
}
    


fn generateCombinations(data: &mut ItemSetData,mut itr:i32){
    let mut candidatesTemp:Vec<String> = Vec::new();
    let mut s1:String;
    let mut s2:String;
   

    if itr==2{
        for i in 0..data.candidates.len(){

            let mut strTokenizer1 = FilteredTokenizer::new(filters::DefaultFilter{}, data.candidates.get(i).unwrap());
            s1 = strTokenizer1.next().unwrap().term().to_string();
            
            for j in i+1..data.candidates.len(){
                let mut strTokenizer2 = FilteredTokenizer::new(filters::DefaultFilter{}, data.candidates.get(j).unwrap());
				s2 = strTokenizer2.next().unwrap().term().to_string();
                let addString:String = String::from(s1.to_string() + " " + &s2.to_string());
                candidatesTemp.push(addString);
            }  
        }
    }else{
        for i in 0..data.candidates.len(){
            for j in i+1..data.candidates.len(){
                s1 = String::new();
                s2 = String::new();

                let mut strTokenizer1 = FilteredTokenizer::new(filters::DefaultFilter{}, data.candidates.get(i).unwrap());
                let mut strTokenizer2 = FilteredTokenizer::new(filters::DefaultFilter{}, data.candidates.get(j).unwrap());

                for s in 0..itr-2{
                    s1 = s1 + " " + &strTokenizer1.next().unwrap().term().to_string();
                    s2 = s2 + " " + &strTokenizer2.next().unwrap().term().to_string();
                }

                if s2.eq_ignore_ascii_case(&s1){
                    let addString:String = (s1 + " " + &strTokenizer1.next().unwrap().term().to_string() + " " + &strTokenizer2.next().unwrap().term().to_string()).trim().to_string();
                    candidatesTemp.push(addString);
                }
            }  
        }
    }
    data.candidates.clear();
    data.candidates = candidatesTemp.clone(); 
    candidatesTemp.clear();
}

fn checkFrequentItems(data:&mut ItemSetData){

    let mut combList:Vec<String> = Vec::new();
    for candidate in &data.candidates{
        combList.push(candidate.to_string())
    } 
    println!("Frequent Itemset: {}", data.displayFrequentItemSetNumber);

    for str in combList{
        let mut flag = 0;
        let mut itemSetOcurrence:usize = 0;
        let words:Vec<String> = str.split_whitespace().map(|x| x.to_string()).collect::<Vec<String>>();
        let count = words.len();

        for transaction in &data.transactions{ 

            for wordStr in &words{ 
                if transaction.contains(wordStr){
                    flag+=1;
                }
            }
            if flag==count{
                itemSetOcurrence+=1;
            }
            flag=0;
        }

        if itemSetOcurrence >= data.minsup{
            println!("{} => support = {}", str, itemSetOcurrence);
            data.frequentItems.insert(str.clone(), itemSetOcurrence);
            data.finalFrequentItemSet.push(str.clone());
        }
        itemSetOcurrence = 0;
    }
    data.displayFrequentItemSetNumber += 1;

}

fn generateAssociationRules(data:&mut ItemSetData){
    let mut confidence:f32;
    let mut confidence2:f32;

    for item in &data.finalFrequentItemSet{ 
        let value = data.frequentItems.get(item).unwrap();
        let mut str:String = String::new();
        let mut str1:String = String::new();
        let words:Vec<String> = item.split_whitespace().map(|x| x.to_string()).collect::<Vec<String>>();
        let wordCountInString = words.len();

        if wordCountInString == 2{
            let s = data.frequentItems.get(&words[0]).unwrap();
            confidence = (*value as f32/ *s as f32);

            if confidence>=data.minconf{
                println!("{} -> {} = Confidence = {} and Support = {}", words[0], words[1], confidence*100 as f32, value); 
            }
            let s1 = data.frequentItems.get(&words[1]).unwrap();
            confidence = (*value as f32/ *s1 as f32);

            if confidence>=data.minconf{
                println!("{} -> {} = Confidence = {} and Support = {}", words[1], words[0], confidence*100 as f32, value);
            }
            
        }else{
            for i in 0..wordCountInString-1{

                if i == 0{
                    str = str + &words[i];
                }else{
                    str = str + " " + &words[i];
                }

                for j in i+1..wordCountInString{
                    str1 = str1 + " " + &words[j];
                }
                let s = data.frequentItems.get(&str).unwrap();
                confidence = (*value as f32/ *s as f32);
                let st:String = str1.trim().to_string();
                let s1 = data.frequentItems.get(&st).unwrap();
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
}  