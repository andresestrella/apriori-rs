use std::{env, io::{BufReader, self, prelude::*}, slice::SliceIndex, collections::HashMap, ops::Add};
use std::fs::File;

pub struct ItemSetData{
    pub transactions:Vec<String>,
    pub candidates:Vec<String>,
    pub itemSet:Vec<String>,
    pub finalFrequentItemSet:Vec<String>,
    pub frequentItems:HashMap<String,i32>,
    pub itemsCount:usize, countItemOccurrence:i32, displayFrequentItemSetNumber:i32, minsup:i32,minconf:i32
}

fn main() -> io::Result<()> {

    // how can i error handle missing arguments here?
    let args: Vec<String> = env::args().collect(); //arg 1 is minsup and arg 2 is min confidence
    let fname = &args[1]; //.unwrap()
    let _min_sup:f32 = args[2].parse().expect("missing min supp argument"); //.unwrap()
    let _min_conf:f32 = args[3].parse().expect("missing min confidence argument");

    let file = File::open(fname)?;
    let reader = BufReader::new(file);

    //find a way to declare vec and collect lines in a single line
    //let transactions:Vec<String> = reader.lines().collect::<Result<_,_>>().unwrap();
    //let no_of_transactions = transactions.len();
    //let itemSet:Vec<String> = Vec::new();
    //let finalFrequentItemSet:Vec<String> = Vec::new();
    //let mut frequentItems:HashMap<String,i32> = HashMap::new();

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

    //display(transactions, no_of_transactions,_min_sup,_min_conf);
    display(&mut appData);
    Ok(())
}

//fn display (transactions:Vec<String>, no_of_transactions:usize, minsup:f32, minconf:f32){
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
        let words:Vec<String> = string.split_whitespace().map(|x| x.to_string()).collect::<Vec<String>>();
        print!("{:?} 3 \n", words); */
    }
    data.itemsCount = data.itemSet.len();
    data.itemSet.sort();
    println!("Item set: {:?}, delete this bruh line 69", data.itemSet);
    println!("Number of items{}", data.itemsCount);
    println!("Number of transactions{}", data.transactions.len());

    firstFrequentItemSet(data);
}

fn firstFrequentItemSet(data:&mut ItemSetData){
    println!("Frequent ItemSet 1");
    for item in &data.itemSet{
        data.countItemOccurrence = 0;
        //let itemStr = data.itemSet.get(i).unwrap();

        for j in 0..data.transactions.len(){

            let transactionStr = data.transactions.get(j).unwrap();

            if transactionStr.contains(item){
                data.countItemOccurrence+=1;
            }

        }

        if data.countItemOccurrence >= data.minsup{
            println!();
            data.finalFrequentItemSet.push(item.clone());
            data.frequentItems.insert(item.to_string(), data.countItemOccurrence);
        }

    }
    aprioriStart(data);

fn aprioriStart(data:&mut ItemSetData){}
    let mut itemSetNumber = 1;

    for set in &data.finalFrequentItemSet{
        data.candidates.push(set.to_string());
    }

    loop{
        itemSetNumber += 1;
        generateCombinations(data, itemSetNumber);
        checkFrequentItems(data);

        if data.candidates.len() <= 1{
            break;
        }
    }

    println!("Association Rules for Frequent Itemset");
    generateAssociationRules(data);
}

fn generateCombinations(data: &mut ItemSetData,mut itr:i32){
    let candidatesTemp:Vec<String> = Vec::new();
    let s1:String;
    let s2:String;
    //StringTokenizers 1 y 2


    if itr==2{
        for i in 0..data.candidates.len(){
            //strToken1 =
            //s1 = strToken1.next();
            for j in i+1..data.candidates.len(){
                //strToken2 = new ;
				//s2 = strToken2.nextToken();
                let addString:String = String::from(&s1.to_string()+" "+&s2.to_string());
                candidatesTemp.push(addString);
            }  
        }
    }else{
        for i in 0..data.candidates.len(){
            for j in i+1..data.candidates.len(){

                s1 = String::new();
                s2 = String::new();

                //strToken1 = ;
                //strToken2 = ;

                for s in 0..itr-2{
                    //s1 = s1 + " " + strToken.nextToken();
                    //s2 = s2 + " " + strToken2.nextToken();
                }

                if !s2.eq_ignore_ascii_case(&s1){ //=0 for false i guess
                    //let addString:String = (s1 + " " + strToken1.nextToken() + " " + strToken2.nextToken()).trim();
                    candidatesTemp.push(addString);
                }
                

            }  
        }

    }
}

fn checkFrequentItems(data:&mut ItemSetData){}

fn generateAssociationRules(data:&mut ItemSetData){}  