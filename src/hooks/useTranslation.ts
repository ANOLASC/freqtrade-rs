import { useTranslation as useI18NextTranslation } from 'react-i18next';
import { useLanguage } from '../contexts/LanguageContext';

export const useTranslation = () => {
  const { language } = useLanguage();
  const { t, i18n } = useI18NextTranslation();

  const changeLanguage = (lang: 'en' | 'zh') => {
    i18n.changeLanguage(lang);
  };

  return { t, i18n, language, changeLanguage };
};
