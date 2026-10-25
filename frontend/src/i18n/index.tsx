import { createContext, useContext, useState, useEffect, ReactNode } from 'react';
import { en } from './en';
import { pt } from './pt';
import { Language } from '../types/models';

type Translations = typeof en;

export type TranslationMap = Translations & ((key: keyof Translations) => string);

interface I18nContextType {
  lang: Language;
  t: TranslationMap;
  toggleLang: () => void;
  setLang: (lang: Language) => void;
}

const I18nContext = createContext<I18nContextType | null>(null);

export function I18nProvider({ children }: { children: ReactNode }) {
  const [lang, setLangState] = useState<Language>(() => {
    const saved = localStorage.getItem('kame_house_lang');
    return saved === 'pt' ? 'pt' : 'en';
  });

  const setLang = (newLang: Language) => {
    setLangState(newLang);
    localStorage.setItem('kame_house_lang', newLang);
    fetch('/api/settings', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ language: newLang === 'pt' ? 'pt-BR' : 'en' }),
    }).catch(() => {});
  };

  const toggleLang = () => {
    setLang(lang === 'en' ? 'pt' : 'en');
  };

  const activeDict = lang === 'pt' ? (pt as Translations) : en;
  const fn = (key: keyof Translations) => activeDict[key] || (key as string);
  const t = Object.assign(fn, activeDict) as TranslationMap;

  return (
    <I18nContext.Provider value={{ lang, t, toggleLang, setLang }}>
      {children}
    </I18nContext.Provider>
  );
}

export function useTranslation() {
  const ctx = useContext(I18nContext);
  if (!ctx) {
    throw new Error('useTranslation must be used within an I18nProvider');
  }
  return ctx;
}
