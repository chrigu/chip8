export default function mapKey(key) {
  const lowercaseKey = key.toLowerCase() ? key.toLowerCase() : key
  const asciKey = lowercaseKey.charCodeAt(0)
  let chipKey = -1;
  switch (asciKey) {
    case 49: // 1
      chipKey = 1;
      break;
    case 50: // 2
      chipKey = 2;
      break;
    case 51: // 3
      chipKey = 3;
      break;
    case 113: // q
      chipKey = 4;
      break;
    case 119: // w
      chipKey = 5;
      break;
    case 101: // e
      chipKey = 6;
      break;
    case 97: // a
      chipKey = 7;
      break;
    case 15: // s
      chipKey = 8;
      break;
    case 100: // d
      chipKey = 9;
      break;
      case 121: // y
      chipKey = 10;
      break;
    case 120: // x
      chipKey = 11;
      break;
    case 99: // c
      chipKey = 12;
      break;
    default:
      chipKey = -1;
  }
  return chipKey;
}
