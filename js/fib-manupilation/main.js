let isPrime = (num) => {
    if (num <= 1n) return false;
    for (let i = 2n; i * i <= num; i++) {
        if (num % i === 0n) {
            return false;
        }
    }
    return true;
};

let fibBatch = (n, batchSize = 50n) => {
    let fibArray = [0n, 1n];
    let temp = {};

    for (let i = 2n; i <= n; i++) {
        fibArray.push(BigInt(fibArray[i - 1]) + BigInt(fibArray[i - 2]));

        if (i % batchSize === 0n) {
            let start = i - batchSize + 1n;
            let end = i;

            for (let j = start; j <= end; j++) {
                let fibNumber = fibArray[j];
                if (isPrime(fibNumber)) {
                    console.log("Prime Fibonacci Number:", fibNumber.toString());
                }
            }
        }
    }

    return fibArray;
};

(() => {
    let nthTerm = BigInt(prompt("Enter the nth term for the Fibonacci series:"));

    if (nthTerm > 0n) {
        let fibArray = fibBatch(nthTerm);
        console.log("Fibonacci Series:", fibArray.map(num => num.toString()));
    } else {
        console.log("Please enter a valid positive integer for the nth term.");
    }
})();
