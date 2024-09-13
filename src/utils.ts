/**
 * Pads a CIK number to 10 digits with leading zeros.
 * @param cik - The CIK number as a string or number.
 * @returns A string with the CIK padded to 10 digits.
 */
export function padCik(cik: string | number): string {
  return String(cik).padStart(10, '0');
}
