function getPermutations(input) {
    const result = new Set();

    function generatePermutations(current, remaining) {
        if (current.length > 0) {
            result.add(parseInt(current));
        }

        for (let i = 0; i < remaining.length; i++) {
            generatePermutations(current + remaining[i], remaining.slice(0, i) + remaining.slice(i + 1));
        }
    }

    generatePermutations('', input.toString());
    return Array.from(result);
}

function isPrime(n) {
    if (n <= 1) {
        return false;
    } else if (n <= 3) {
        return true;
    } else if (n % 2 === 0 || n % 3 === 0) {
        return false;
    }

    let i = 5;
    while (i * i <= n) {
        if (n % i === 0 || n % (i + 2) === 0) {
            return false;
        }
        i += 6;
    }

    return true;
}

const userInput = prompt("Enter a number of at least 4 digits:");
const inputNumber = parseInt(userInput);

if (isNaN(inputNumber) || inputNumber < 1000) {
    console.log("Invalid input. Please enter a number of at least 4 digits.");
} else {
    const permutations = getPermutations(inputNumber);
    const primeNumbers = permutations.filter(num => isPrime(num));

    // Find the maximum number for stopping the triangle
    const maxNumber = Math.max(...permutations);

    // Display All Combinations
    const allCombinationsContainer = document.createElement("div");
    allCombinationsContainer.classList.add("all-combinations");
    allCombinationsContainer.innerHTML = "<h3>All Combinations</h3>";

    const allCombinationsRow = document.createElement("div");
    allCombinationsRow.classList.add("row");

    permutations.forEach(number => {
        const cell = document.createElement("div");
        cell.classList.add("cell");
        cell.innerHTML = number;
        allCombinationsRow.appendChild(cell);
    });

    allCombinationsContainer.appendChild(allCombinationsRow);
    document.body.appendChild(allCombinationsContainer);

    // Display All Prime Numbers
    const allPrimesContainer = document.createElement("div");
    allPrimesContainer.classList.add("all-primes");
    allPrimesContainer.innerHTML = "<h3>All Prime Numbers</h3>";

    const allPrimesRow = document.createElement("div");
    allPrimesRow.classList.add("row");

    primeNumbers.forEach(number => {
        const cell = document.createElement("div");
        cell.classList.add("cell", "prime");
        cell.innerHTML = number;
        allPrimesRow.appendChild(cell);
    });

    allPrimesContainer.appendChild(allPrimesRow);
    document.body.appendChild(allPrimesContainer);

    // Create Pascal's Triangle
    const triangle = [];
    for (let i = 0; i < inputNumber; i++) {
        triangle[i] = [];
        for (let j = 0; j <= i; j++) {
            if (j === 0 || j === i) {
                triangle[i][j] = 1;
            } else {
                triangle[i][j] = triangle[i - 1][j - 1] + triangle[i - 1][j];
            }
        }

        // Stop when the maximum number is reached
        if (Math.max(...triangle[i]) > maxNumber) {
            break;
        }
    }

    // Display Pascal's Triangle
    const triangleContainer = document.createElement("div");
    triangleContainer.classList.add("triangle");

    for (let i = 0; i < triangle.length; i++) {
        const row = document.createElement("div");
        row.classList.add("row");
        for (let j = 0; j < triangle[i].length; j++) {
            const cell = document.createElement("div");
            cell.classList.add("cell");
            cell.innerHTML = triangle[i][j];
            if (primeNumbers.includes(triangle[i][j])) {
                cell.classList.add("prime");
            }
            row.appendChild(cell);
        }
        triangleContainer.appendChild(row);
    }

    document.body.appendChild(triangleContainer);

    // Display Error Ratio
    const notHighlightedPrimes = primeNumbers.filter(num => !triangle.flat().includes(num));
    const errorRatio = notHighlightedPrimes.length / primeNumbers.length;

    const errorRatioContainer = document.createElement("div");
    errorRatioContainer.classList.add("error-ratio");
    errorRatioContainer.innerHTML = "<h3>Error Ratio:</h3>";

    const ratioCell = document.createElement("div");
    ratioCell.classList.add("prime");
    ratioCell.innerHTML = "Error Ratio: " + errorRatio.toFixed(2);
    errorRatioContainer.appendChild(ratioCell);
    document.body.appendChild(errorRatioContainer);

    // Display Prime Numbers not Highlighted
    const notHighlightedPrimesContainer = document.createElement("div");
    notHighlightedPrimesContainer.classList.add("not-highlighted-primes");
    notHighlightedPrimesContainer.innerHTML = "<h3>Prime Numbers Not Highlighted</h3>";

    const notHighlightedPrimesRow = document.createElement("div");
    notHighlightedPrimesRow.classList.add("row");

    notHighlightedPrimes.forEach(number => {
        const cell = document.createElement("div");
        cell.classList.add("cell", "prime");
        cell.innerHTML = number;
        notHighlightedPrimesRow.appendChild(cell);
    });

    notHighlightedPrimesContainer.appendChild(notHighlightedPrimesRow);
    document.body.appendChild(notHighlightedPrimesContainer);
}