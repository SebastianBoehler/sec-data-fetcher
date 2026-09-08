import { createHttpClient } from '../src/config';

it('bounds requests and throttles dispatch within an interval', async () => {
  jest.useFakeTimers();
  const http = createHttpClient(1, 1000);
  const adapter = jest.fn(async (config) => ({
    data: 'ok',
    status: 200,
    statusText: 'OK',
    headers: {},
    config,
  }));
  http.defaults.adapter = adapter;
  try {
    expect(http.defaults.timeout).toBe(30_000);
    const first = http.get('https://example.com/first');
    const second = http.get('https://example.com/second');
    await jest.advanceTimersByTimeAsync(1);
    expect(adapter).toHaveBeenCalledTimes(1);
    await jest.advanceTimersByTimeAsync(999);
    await Promise.all([first, second]);
    expect(adapter).toHaveBeenCalledTimes(2);
  } finally {
    jest.useRealTimers();
  }
});
