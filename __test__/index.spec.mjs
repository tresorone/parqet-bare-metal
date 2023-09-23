import test from 'ava';

import { getXirr } from '../index';

test('xirr', t => {
  const today = new Date();
  const years = Array.from({ length: 6 }).map(
    (_, idx) => today.getFullYear() - idx
  );
  const xirr = getXirr(
    [-1000, 100, 100, 100, 100, 1000],
    years.map(n => new Date(n).getTime())
  );

  t.is(xirr, 0);
});
