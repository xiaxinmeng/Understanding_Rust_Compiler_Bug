 python
import logging
logging.configure({
  'handlers': {
    'file': {
      'class': 'logging.FileHandler',
      'file': '/var/log/myapp.log'
    },
    'email': {
      'class': 'logging.EmailHandler',
      'level': 'ERROR',
      // from, to, etc
    }
  },
  'loggers': {
    'myapp': {
      'level': 'INFO',
      'handlers': ['file', 'email']
    },
    'mysql2': {
      'level': 'ERROR',
      'handlers': ['file']
    }
  }
})
