import axios from 'axios';

const api = axios.create({
  baseURL: import.meta.env.VITE_API_URL,
});

export const register = (data: {
  email: string;
  password: string;
  name?: string;
}) => api.post('/register', data);

export const login = (data: {
  email: string;
  password: string;
}) => api.post('/login', data);

export const requestPasswordReset = (data: {
  email: string;
}) => api.post('/request-password-reset', data);

export const resetPassword = (data: {
  token: string;
  newPassword: string;
}) => api.post('/reset-password', data);