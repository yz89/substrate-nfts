import bottle from './static/bottle.png';
import cup from './static/cup.png';
import mug from './static/mug.png';
import honey from './static/honey.png';
import hoodie from './static/hoodie.png';
import phone from './static/phone.png';
import {NFTokenId}from './types';
const items = [
  {
    text: '100: Bottle Water',
    value: 100,
    name: 'Bottle Water',
    image: bottle,
    token_id:new NFTokenId(0)
  },
  {
    text: '101: Coffee',
    value: 101,
    name: 'Coffee',
    image: cup,
    token_id:new NFTokenId(1)
  },
  {
    text: '200: Mug',
    value: 200,
    name: 'Mug',
    image: mug,
    token_id:new NFTokenId(2)
  },
  {
    text: '201: Honey',
    value: 201,
    name: 'Honey',
    image: honey,
    token_id:new NFTokenId(3)
  },
  {
    text: '300: Hoodie',
    value: 300,
    name: 'Hoodie',
    image: hoodie,
    token_id:new NFTokenId(4)
  },
  {
    text: '301: Phone',
    value: 301,
    name: 'Phone',
    image: phone,
    token_id:new NFTokenId(5)
  }
];

export default items;
