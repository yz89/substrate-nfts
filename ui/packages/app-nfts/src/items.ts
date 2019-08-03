import bottle from './static/bottle.png';
import cup from './static/cup.png';
import mug from './static/mug.png';
import honey from './static/honey.png';
import hoodie from './static/hoodie.png';
import phone from './static/phone.png';

const items = [
  {
    text: '100: Bottle Water',
    value: 100,
    name: 'Bottle Water',
    image: bottle,
    token_id:[0,155,3,136,178,163,188,219,106,53,167,25,55,106,194,177,118,131,196,93,142,233,131,89,181,196,200,68,132,191,96,73]
  },
  {
    text: '101: Coffee',
    value: 101,
    name: 'Coffee',
    image: cup,
    token_id:[231,212,81,242,99,35,120,23,114,244,107,250,172,52,172,183,3,96,72,184,79,4,77,92,54,115,179,226,88,50,77,49]
  },
  {
    text: '200: Mug',
    value: 200,
    name: 'Mug',
    image: mug,
    token_id:[59,219,11,71,9,24,254,26,147,218,94,3,177,104,163,65,239,135,98,76,10,47,144,235,176,211,62,106,204,84,66,82]
  },
  {
    text: '201: Honey',
    value: 201,
    name: 'Honey',
    image: honey,
    token_id:[216,30,103,125,160,51,221,200,16,224,213,201,103,165,166,207,35,119,93,23,106,204,245,47,18,148,188,136,37,106,90,157]
  },
  {
    text: '300: Hoodie',
    value: 300,
    name: 'Hoodie',
    image: hoodie,
    token_id:[232,228,56,206,134,137,19,13,145,186,138,149,209,228,94,251,214,183,73,80,0,120,239,201,35,216,7,71,126,168,55,98]
  },
  {
    text: '301: Phone',
    value: 301,
    name: 'Phone',
    image: phone,
    token_id:[105,32,194,81,1,90,127,48,19,91,0,9,40,91,127,202,76,10,234,21,252,146,139,211,93,38,254,49,177,112,195,114]
  }
];

export default items;
