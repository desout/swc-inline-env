const expr = "production" !== 'production'

const result = process.env.NODE_ENV_NOT_EXIST === 'production';

if (!process.env.WEBPACK_ENV) {
    const expr = 'test';
} 