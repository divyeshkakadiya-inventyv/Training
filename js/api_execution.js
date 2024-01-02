(() => {
    let arr = [1,2,3,4,5]
    let fc = arr.shift();
    f2(fc , ...arr)
})(); //iffe

function f2(e , ...arr){
    let arr2 = [6,7,8,9]
    arr2.unshift(e)
    arr2 = arr2.concat(...arr)

    let promise = new Promise((resolve , reject) => {
        let sum = arr2.reduce((e , sum)=>e + sum)
        sum > 30 ? resolve(sum) : reject();
    })

    promise.then((e)=>{
        fetch(`https://jsonplaceholder.typicode.com/photos?_limit=${e}`)
        .then(response => response.json())
        .then(json => {
            for(let i = 0 ; i< json.length ; i++){
                console.log(i+1 , json[i].url);
            }
        })
    }).catch(()=>{
        console.log("not greater than 30")
    })
}
