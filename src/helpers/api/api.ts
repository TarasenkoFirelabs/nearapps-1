export type ResponseError = {
    error: string;
};

const defaultHeaders = {
    'Content-type': 'application/json',
    'Client-Platform': 'WEB_BOOKING',
    'Client-Version': '1.0.0',
};

/**
 * Calls api.
 *
 * @param path - The api url
 * @param init - The request options
 */
export const callApi = async (path: string, init?: RequestInit) => {
    const userData = localStorage.getItem('auth');
    const token: string = userData ? JSON.parse(userData).token : '';

    const authHeaders: Record<string, string> = token
        ? { Authorization: `Bearer ${token}` }
        : {};

    return await fetch(path, {
        ...init,
        headers: {...defaultHeaders, ...authHeaders, ...init?.headers},
    });
};

/**
 * Calls POST api.
 * @param path - The api url
 * @param body - The request body
 * @param init - The request options
 */
export const postApi = <B>(path: string, body: B, init?: RequestInit) => {
    return callApi(path, { method: 'POST', body: JSON.stringify(body), ...init });
};

export const patchApi = async (path: string, body: any[]) => {
    const userData = localStorage.getItem('auth');
    const token: string = userData ? JSON.parse(userData).token : '';

    const patchHeaders = {
        ...defaultHeaders,
        'Content-type': 'application/json-patch+json',
    };

    const authHeaders = token ? { Authorization: `Bearer ${token}` } : {};

    return await fetch(path, {
        headers: token ? {...patchHeaders, ...authHeaders} : patchHeaders,
        method: 'PATCH',
        body: JSON.stringify(body),
    });
};

export const deleteApi = (path: string, init?: RequestInit) => {
    return callApi(path, { method: 'DELETE', ...init });
};
