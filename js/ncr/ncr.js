let fact = (n) => {
    if(n == 1){
        return n;
    }
    return n*fact(n-1)
}

let combination = (n , r) => {
    console.log(fact(n) / (fact(r) * fact(n-r)))
}

(()=>{
    let n = parseInt(prompt("Enter n : "));
    let r = parseInt(prompt("Enter r : "))
    combination(n , r)
})()


