import * as fs from 'fs';

const main = () => {
    let numOfLines =0 
const initialValue = 0;
    let temp:any = []
    console.log("test")
    fs.readFileSync(process.argv[2]).toString()
        .split("\n")
        .forEach((line) => {
          

            numOfLines++
            temp.push(line.length)
            console.log(line)
        });

    let wc = temp.reduce((accumulator: number, currentValue: number) => accumulator + currentValue, initialValue)
    console.log(numOfLines) 
    console.log(wc)      
}

main()