const LETTER_SCORES = {
  A: 1,
  E: 1,
  I: 1,
  O: 1,
  U: 1,
  L: 1,
  N: 1,
  R: 1,
  S: 1,
  T: 1,
  B: 3,
  C: 3,
  M: 3,
  P: 3,
  G: 2,
  D: 2,
  F: 4,
  H: 4,
  V: 4,
  W: 4,
  Y: 4,
  K: 5,
  X: 8,
  J: 8,
  Z: 10,
  Q: 10,
};

const letterToScore = (letter) => LETTER_SCORES[letter.toUpperCase()];

const sum = () => (acc, next) => acc + next;

export const score = (word) => {
  return word.split("").map(letterToScore).reduce(sum(), 0);
};
