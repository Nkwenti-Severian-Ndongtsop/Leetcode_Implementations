function palindromeNumber(integer){
        return integer.toString() === integer.toString().split('').reverse().join('');
}
console.log(palindromeNumber(121));