let add = (n , arr) => {
    if(n == 0) return;
    let e = parseInt(prompt("Enter element:"))
    arr.push(e)
    // console.log(arr)
    return add(n-1 , arr)
}
(()=>{
    let arr = []
    let n = parseInt(prompt("How much element you want to add in array:"))
    add(n , arr)
    console.log(arr);
})()

