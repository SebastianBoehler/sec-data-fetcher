/**
 * Pads a CIK number to 10 digits with leading zeros.
 * @param cik - The CIK number as a string or number.
 * @returns A string with the CIK padded to 10 digits.
 */
export function padCik(cik: string | number): string {
  const value = String(cik).trim();
  if (!/^\d{1,10}$/.test(value))
    throw new TypeError('CIK must contain 1 to 10 digits.');
  return value.padStart(10, '0');
}
