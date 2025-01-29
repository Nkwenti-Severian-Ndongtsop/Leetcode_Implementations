// A function to get the fibonacci sequence
function fibonacciSequence(integer){
    sequence = [0,1];
    if (integer === 0){
      return 0
    } else for ( i=2; i<=integer; i++ ){
      sequence[i] = sequence[i-1] + sequence[i-2]
    }
return sequence;
}
console.log(fibonacciSequence(20));
