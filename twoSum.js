function twoSum(numbers, target) {
let result = [];
const n = numbers.length;
    for ( i=0; i<n; i++ ) {
      for ( j=i+1; j<n; j++ ) {
        if (numbers[i] === target - numbers[j]) {
          result = [numbers[i], numbers[j]];
        }
      }
    }
    return result;
}
console.log(twoSum([numbers], target));