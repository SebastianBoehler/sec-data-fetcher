import axios from 'axios';
import rateLimit from 'axios-rate-limit';

/**
 * Creates a rate-limited Axios instance.
 * @param maxRequests - Maximum number of requests per interval.
 * @param perMilliseconds - Interval in milliseconds.
 * @returns A rate-limited Axios instance.
 */
export const createHttpClient = (
  maxRequests: number = 10,
  perMilliseconds: number = 1000,
) => {
  return rateLimit(axios.create(), {
    maxRequests,
    perMilliseconds,
  });
};

/**
 * Generates headers for SEC requests.
 * @param host - The host of the SEC endpoint.
 * @param userAgent - Your company's user agent string.
 * @returns An object containing the headers.
 */
export const getHeaders = (host: string, userAgent: string) => ({
  'User-Agent': userAgent,
  'Accept-Encoding': 'gzip, deflate',
  Host: host,
});
